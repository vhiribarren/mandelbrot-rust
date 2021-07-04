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

impl display::SdlPixelProvider for mandelbrot::MandelbrotContext {
    fn width(&self) -> u32 {
        self.width as u32
    }
    fn height(&self) -> u32 {
        self.height as u32
    }
    fn compute_pixel(&self, x: u32, y: u32) -> sdl2::pixels::Color {
        let result = self.color_at_pixel(x, y);
        sdl2::pixels::Color {
            r: result,
            g: result,
            b: result,
            a: 255,
        }
    }
}

fn main() {
    let mandel_ctx = mandelbrot::MandelbrotContext {
        width: 800,
        height: 600,
        upper_left: num::Complex { re: -1.0, im: 1.0 },
        lower_right: num::Complex { re: 1.0, im: -1.0 },
        limit: 255,
    };
    display::render_sdl(mandel_ctx).unwrap();
}
