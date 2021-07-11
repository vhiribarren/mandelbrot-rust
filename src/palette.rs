use sdl2::pixels::Color;

pub fn generate_palette_gradient_bicolor(count: usize, color_start: Color, color_end: Color) -> Vec<Color> {
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



pub fn generate_palette_gradient_multiple(count: usize, colors: &Vec<(Color, f64)>) -> Vec<Color> {
    assert!(colors.len() > 1, "There must be at least 2 colors");
    assert_eq!(colors.first().unwrap().1, 0., "First color placement must be 0");
    assert_eq!(colors.last().unwrap().1, 1., "First color placement must be 1");
    let mut palette = Vec::with_capacity(count);
    let mut color_index = 0;
    let mut color_start= &colors[color_index].0;
    let mut color_end= &colors[color_index+1].0;
    for i in 0..count {
        let current_ratio = (i as f64) / (count as f64);
        if current_ratio >= colors[color_index+1].1 {
            color_index += 1;
            color_start = &colors[color_index].0;
            color_end = &colors[color_index+1].0;
        }
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
