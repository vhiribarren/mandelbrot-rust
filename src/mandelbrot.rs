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

use num::Complex;

pub struct MandelbrotContext {
    pub width: usize,
    pub height: usize,
    pub upper_left: Complex<f64>,
    pub lower_right: Complex<f64>,
    pub limit: usize,
}

impl MandelbrotContext {
    pub fn color_at_pixel(&self, x: u32, y: u32) -> usize {
        let mandelbrot_point = self.point_at_pixel(x, y);
        let set_membering = Self::in_mandelbrot_set(mandelbrot_point, self.limit);
        match set_membering {
            Err(val) => val,
            _ => self.limit - 1,
        }
    }

    pub fn point_at_pixel(&self, x: u32, y: u32) -> Complex<f64> {
        Complex {
            re: self.upper_left.re
                + (x as f64 / self.width as f64) * (self.lower_right.re - self.upper_left.re),
            im: self.upper_left.im
                - (y as f64 / self.height as f64) * (self.upper_left.im - self.lower_right.im),
        }
    }

    fn usize_to_u8(&self, val: usize) -> u8 {
        (255_f64 * (val as f64) / (self.limit as f64)) as u8
    }

    /// Check if c is in the Mandelbrot set
    ///
    /// It is a success if the result is Ok, otherwise the number
    /// of iteration up to the divergence is returned in Err.
    fn in_mandelbrot_set(c: Complex<f64>, limit: usize) -> Result<(), usize> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..(limit - 1) {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return Err(i);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pixel_to_point() {
        let bounds = MandelbrotContext {
            width: 100,
            height: 200,
            upper_left: Complex { re: -1.0, im: 1.0 },
            lower_right: Complex { re: 1.0, im: -1.0 },
            limit: 255,
        };
        assert_eq!(bounds.point_at_pixel(0, 0), Complex { re: -1.0, im: 1.0 });
        assert_eq!(
            bounds.point_at_pixel(100, 200),
            Complex { re: 1.0, im: -1.0 }
        );
        assert_eq!(bounds.point_at_pixel(50, 100), Complex { re: 0.0, im: 0.0 });
        assert_eq!(
            bounds.point_at_pixel(25, 175),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
