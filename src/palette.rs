use sdl2::pixels::Color;

pub fn generate_palette(count: usize, color_start: Color, color_end: Color) -> Vec<Color> {
    let mut palette = Vec::with_capacity(count);
    for i in 0..count {
        let current_ratio = (count as f64 - i as f64) / (count as f64);
        let current_color = Color {
            r: (color_start.r as f64 + (color_end.r as f64 - color_start.r as f64) * current_ratio)
                as u8,
            g: (color_start.g as f64 + (color_end.g as f64 - color_start.g as f64) * current_ratio)
                as u8,
            b: (color_start.b as f64 + (color_end.b as f64 - color_start.b as f64) * current_ratio)
                as u8,
            a: 255,
        };
        palette.push(current_color);
    }
    palette
}
