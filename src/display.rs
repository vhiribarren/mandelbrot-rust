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

const SDL_WINDOW_CLEAR_COLOR: Color = Color {
    r: 77,
    g: 77,
    b: 170,
    a: 255,
};

const SELECTION_RECTANGLE_COLOR: Color = Color::RED;

const IDLE_LOOP_SLEEP_DURATION: Duration = Duration::from_millis(50);
const RENDERING_SCREEN_REFRESH_PERIOD: Duration = Duration::from_millis(20);

pub trait PixelProvider {
    fn with_new_bounds(
        self,
        width: u32,
        height: u32,
        point_upper_left: (f64, f64),
        point_lower_right: (f64, f64),
    ) -> Self;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn compute_pixel_color(&self, pixel: Point) -> Color;
    fn point_at_pixel(&self, pixel: Point) -> (f64, f64);
}

enum CanvasSelection {
    None,
    OnGoing(Rect),
    Selected(Rect),
}

struct MouseSelection {
    start_x: i32,
    start_y: i32,
    in_selection: bool,
    selection: Rect,
}

impl MouseSelection {
    fn new() -> MouseSelection {
        MouseSelection {
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
        let delta_y = mouse_state_y - self.start_y;
        let origin_x = if delta_x >= 0 {
            self.start_x
        } else {
            mouse_state_x
        };
        let origin_y = if delta_y >= 0 {
            self.start_y
        } else {
            mouse_state_y
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

pub fn render_sdl(mut pixel_provider: impl PixelProvider) -> Result<(), String> {
    let mut mouse_selection = MouseSelection::new();
    let mut render_canvas = Surface::new(
        pixel_provider.width(),
        pixel_provider.height(),
        PixelFormatEnum::RGBA32,
    )?
    .into_canvas()?;
    render_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
    render_canvas.clear();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Mandelbrot",
            pixel_provider.width(),
            pixel_provider.height(),
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|err| err.to_string())?;

    let mut window_canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;
    window_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
    // Paint and blit back buffer
    window_canvas.clear();
    window_canvas.present();

    let texture_creator = window_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(render_canvas.surface())
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut width_pos: i32 = 0;
    let mut height_pos: i32 = 0;

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

        let is_rendering: bool;
        if (height_pos as u32) < pixel_provider.height()
        {
            is_rendering = true;
            let instant = std::time::Instant::now();
            loop {
                let pixel_gray =
                    pixel_provider.compute_pixel_color(Point::new(width_pos, height_pos));
                render_canvas.set_draw_color(pixel_gray);
                render_canvas.draw_point(Point::new(width_pos as i32, height_pos as i32))?;
                width_pos += 1;
                if width_pos as u32 >= pixel_provider.width() {
                    width_pos = 0;
                    height_pos += 1;
                    if height_pos as u32 >= pixel_provider.height() {
                        break;
                    }
                }
                if instant.elapsed().gt(&RENDERING_SCREEN_REFRESH_PERIOD) {
                    break;
                }
            }
            texture = texture_creator
                .create_texture_from_surface(render_canvas.surface())
                .map_err(|err| err.to_string())?;
        } else {
            is_rendering = false;
        }

        window_canvas.clear();
        window_canvas.copy(&texture, None, None)?;
        match mouse_selection.update_selection(event_pump.mouse_state()) {
            CanvasSelection::None => {}
            CanvasSelection::OnGoing(rect) => {
                window_canvas.set_draw_color(SELECTION_RECTANGLE_COLOR);
                window_canvas.draw_rect(rect)?;
            }
            CanvasSelection::Selected(rect) => {
                let new_width = pixel_provider.width();
                let new_height = pixel_provider.height();
                let coords_upper_left = pixel_provider.point_at_pixel(rect.top_left());
                let coords_lower_right = pixel_provider.point_at_pixel(rect.bottom_right());
                pixel_provider = pixel_provider.with_new_bounds(
                    new_width,
                    new_height,
                    coords_upper_left,
                    coords_lower_right,
                );
                width_pos = 0;
                height_pos = 0;
            }
        }
        window_canvas.present();
        if !is_rendering {
            std::thread::sleep(IDLE_LOOP_SLEEP_DURATION);
        }
    }

    Ok(())
}
