/*
MIT License
Copyright (c) 2021 Vincent Hiribarren
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use std::time::Duration;

use log::info;
use sdl2::render::{Canvas, WindowCanvas};

const SDL_WINDOW_CLEAR_COLOR: Color = Color {
    r: 77,
    g: 77,
    b: 170,
    a: 255,
};

const SELECTION_RECTANGLE_COLOR: Color = Color::RED;

const IDLE_LOOP_SLEEP_DURATION: Duration = Duration::from_millis(50);
const RENDERING_SCREEN_REFRESH_PERIOD: Duration = Duration::from_millis(20);

#[derive(Clone)]
pub struct CanvasBounds {
    pub width: u32,
    pub height: u32,
    pub upper_left: (f64, f64),
    pub lower_right: (f64, f64),
}

impl CanvasBounds {
    pub fn coords_at_pixel(&self, p: Point) -> (f64, f64) {
        (
            self.upper_left.0
                + (p.x as f64 / self.width as f64) * (self.lower_right.0 - self.upper_left.0),
            self.upper_left.1
                - (p.y as f64 / self.height as f64) * (self.upper_left.1 - self.lower_right.1),
        )
    }
}

pub trait PixelComputeProvider<T: PixelCompute> {
    fn new_pixel_compute(&self, bounds: CanvasBounds) -> T;
}

pub trait PixelCompute {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn compute_pixel_color(&self, pixel: Point) -> usize;
}

enum CanvasSelection {
    None,
    OnGoing(Rect),
    Selected(Rect),
}

struct MouseSelection {
    ratio: f64,
    start_x: i32,
    start_y: i32,
    in_selection: bool,
    selection: Rect,
}

impl MouseSelection {
    fn new(ratio: f64) -> MouseSelection {
        MouseSelection {
            ratio,
            start_x: 0,
            start_y: 0,
            in_selection: false,
            selection: Rect::new(0, 0, 0, 0),
        }
    }
    fn update_selection(&mut self, mouse_state: MouseState) -> CanvasSelection {
        if !mouse_state.left() {
            if self.in_selection {
                self.in_selection = false;
                return CanvasSelection::Selected(self.selection);
            }
            return CanvasSelection::None;
        }
        let mouse_state_x = mouse_state.x();
        let mouse_state_y = mouse_state.y();
        if !self.in_selection {
            self.in_selection = true;
            self.start_x = mouse_state_x;
            self.start_y = mouse_state_y;
        }
        let delta_x = mouse_state_x - self.start_x;
        let sign_y = if mouse_state_y - self.start_y > 0 {
            1_i32
        } else {
            -1_i32
        };
        let delta_y = sign_y * (delta_x.abs() as f64 / self.ratio) as i32;
        let origin_x = if delta_x >= 0 {
            self.start_x
        } else {
            mouse_state_x
        };
        let origin_y = if delta_y >= 0 {
            self.start_y
        } else {
            self.start_y + delta_y
        };
        self.selection = Rect::new(
            origin_x,
            origin_y,
            delta_x.abs() as u32,
            delta_y.abs() as u32,
        );
        CanvasSelection::OnGoing(self.selection)
    }
}

struct PixelRenderer<'a, T> {
    width_max: i32,
    height_max: i32,
    width_pos: i32,
    height_pos: i32,
    canvas: Canvas<Surface<'a>>,
    pixel_provider: T,
    palette: &'a [Color],
}

impl<'a, T: PixelCompute> PixelRenderer<'a, T> {
    fn new(pixel_provider: T, palette: &'a [Color]) -> Self {
        let width_max = pixel_provider.width() as i32;
        let height_max = pixel_provider.height() as i32;
        let mut canvas = Surface::new(width_max as u32, height_max as u32, PixelFormatEnum::RGBA32)
            .unwrap()
            .into_canvas()
            .unwrap();
        canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
        canvas.clear();
        PixelRenderer {
            width_max,
            height_max,
            width_pos: 0,
            height_pos: 0,
            canvas,
            pixel_provider,
            palette,
        }
    }

    fn is_rendering(&self) -> bool {
        self.height_pos < self.height_max
    }

    fn update(&mut self, max_timeslot: &Duration) -> Result<(), String> {
        let instant = std::time::Instant::now();
        loop {
            let pixel_gray = self
                .pixel_provider
                .compute_pixel_color(Point::new(self.width_pos, self.height_pos));
            self.canvas.set_draw_color(self.palette[pixel_gray]);
            self.canvas
                .draw_point(Point::new(self.width_pos as i32, self.height_pos as i32))?;
            self.width_pos += 1;
            if self.width_pos >= self.width_max {
                self.width_pos = 0;
                self.height_pos += 1;
                if self.height_pos >= self.height_max {
                    break;
                }
            }
            if instant.elapsed().gt(&max_timeslot) {
                break;
            }
        }
        Ok(())
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(self.canvas.surface())
            .map_err(|err| err.to_string())?;
        canvas.copy(&texture, None, None)?;
        Ok(())
    }
}

pub fn render_sdl<T>(
    mut bounds: CanvasBounds,
    compute_provider: impl PixelComputeProvider<T>,
    palette: &[Color],
) -> Result<(), String>
where
    T: PixelCompute,
{
    let pixel_compute = compute_provider.new_pixel_compute(bounds.clone());
    let mut pixel_renderer = PixelRenderer::new(pixel_compute, palette);
    let screen_ratio = (bounds.width as f64) / (bounds.height as f64);
    let mut mouse_selection = MouseSelection::new(screen_ratio);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Mandelbrot", bounds.width, bounds.height)
        .position_centered()
        .build()
        .map_err(|err| err.to_string())?;
    let mut window_canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;
    window_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);

    let mut event_pump = sdl_context.event_pump()?;
    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                _ => {}
            }
        }
        if pixel_renderer.is_rendering() {
            pixel_renderer.update(&RENDERING_SCREEN_REFRESH_PERIOD)?;
        }

        window_canvas.clear();
        pixel_renderer.render(&mut window_canvas)?;

        match mouse_selection.update_selection(event_pump.mouse_state()) {
            CanvasSelection::None => {}
            CanvasSelection::OnGoing(rect) => {
                window_canvas.set_draw_color(SELECTION_RECTANGLE_COLOR);
                window_canvas.draw_rect(rect)?;
            }
            CanvasSelection::Selected(rect) => {
                let new_width = bounds.width;
                let new_height = bounds.height;
                let coords_upper_left = bounds.coords_at_pixel(rect.top_left());
                let coords_lower_right = bounds.coords_at_pixel(rect.bottom_right());

                bounds = CanvasBounds {
                    width: new_width,
                    height: new_height,
                    upper_left: coords_upper_left,
                    lower_right: coords_lower_right,
                };
                let pixel_provider = compute_provider.new_pixel_compute(bounds.clone());

                pixel_renderer = PixelRenderer::new(pixel_provider, palette);
                info!(
                    "TopLeft: {:?} - BottomRight: {:?}",
                    coords_upper_left, coords_lower_right
                );
            }
        }

        window_canvas.present();
        if !pixel_renderer.is_rendering() {
            std::thread::sleep(IDLE_LOOP_SLEEP_DURATION);
        }
    }

    Ok(())
}
