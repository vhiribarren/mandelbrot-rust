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
use sdl2::pixels::PixelFormatEnum;

const SDL_WINDOW_CLEAR_COLOR: sdl2::pixels::Color = sdl2::pixels::Color {
    r: 77,
    g: 77,
    b: 170,
    a: 255,
};

pub trait SdlPixelProvider {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn compute_pixel(&self, x: u32, y: u32) -> sdl2::pixels::Color;
}

pub fn render_sdl(pixel_provider: impl SdlPixelProvider) -> Result<(), String> {
    let mut render_canvas = sdl2::surface::Surface::new(
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
    let mut width_pos: u32 = 0;
    let mut height_pos: u32 = 0;

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
        if width_pos < pixel_provider.width() || height_pos < pixel_provider.height() {
            let instant = std::time::Instant::now();
            loop {
                let pixel_gray = pixel_provider.compute_pixel(width_pos, height_pos);
                render_canvas.set_draw_color(pixel_gray);
                render_canvas
                    .draw_point(sdl2::rect::Point::new(width_pos as i32, height_pos as i32))?;
                width_pos += 1;
                if width_pos >= pixel_provider.width() {
                    width_pos = 0;
                    height_pos += 1;
                    if height_pos >= pixel_provider.height() {
                        break;
                    }
                }
                if instant.elapsed().as_millis() > 20 {
                    break;
                }
            }
            texture = texture_creator
                .create_texture_from_surface(render_canvas.surface())
                .map_err(|err| err.to_string())?;
            window_canvas.clear();
            window_canvas.copy(&texture, None, None)?;
            window_canvas.present();
        } else {
            window_canvas.clear();
            window_canvas.copy(&texture, None, None)?;
            window_canvas.present();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    Ok(())
}
