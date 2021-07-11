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

mod display;
mod mandelbrot;
mod palette;

use mandelbrot::MandelbrotContext;

impl display::PixelProvider for MandelbrotContext {
    fn with_new_bounds(
        self,
        width: u32,
        height: u32,
        upper_left: (f64, f64),
        lower_right: (f64, f64),
    ) -> Self {
        MandelbrotContext {
            width: width as usize,
            height: height as usize,
            upper_left: num::Complex {
                re: upper_left.0,
                im: upper_left.1,
            },
            lower_right: num::Complex {
                re: lower_right.0,
                im: lower_right.1,
            },
            limit: self.limit,
        }
    }
    fn width(&self) -> u32 {
        self.width as u32
    }
    fn height(&self) -> u32 {
        self.height as u32
    }
    fn compute_pixel_color(&self, pixel: sdl2::rect::Point) -> usize {
        self.color_at_pixel(pixel.x() as u32, pixel.y() as u32) as usize
    }
    fn point_at_pixel(&self, point: sdl2::rect::Point) -> (f64, f64) {
        let coords = self.point_at_pixel(point.x() as u32, point.y() as u32);
        (coords.re, coords.im)
    }
}

fn main() {
    let limit = 1000;
    let color_green = sdl2::pixels::Color {
        r: 0,
        g: 200,
        b: 0,
        a: 255,
    };
    let color_blue_dark = sdl2::pixels::Color {
        r: 0,
        g: 0,
        b: 100,
        a: 255,
    };
    let screen_ratio = 16_f64 / 9_f64;
    let screen_width = 1280;
    let mandelbrot_x_min = -2.5;
    let mandelbrot_x_max = 1.5;
    let mandelbrot_y_max = (mandelbrot_x_max - mandelbrot_x_min) / screen_ratio / 2.;
    let mandel_ctx = MandelbrotContext {
        width: screen_width,
        height: (screen_width as f64 / screen_ratio) as usize,
        upper_left: num::Complex {
            re: mandelbrot_x_min,
            im: mandelbrot_y_max,
        },
        lower_right: num::Complex {
            re: mandelbrot_x_max,
            im: -mandelbrot_y_max,
        },
        limit,
    };
    //let palette = palette::generate_palette_gradient_bicolor(limit, color_start, color_end);
    let color_gradient = vec![
        (color_blue_dark, 0.),
        (sdl2::pixels::Color::WHITE, 0.4),
        (sdl2::pixels::Color::BLACK, 0.5),
        (color_green, 0.6),
        (color_blue_dark, 1.)];
    let palette = palette::generate_palette_gradient_multiple(limit, &color_gradient);
    display::render_sdl(mandel_ctx, &palette).unwrap();
}
