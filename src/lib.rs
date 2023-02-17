use std::fmt::Write;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_points(line_length: f64, scale: f32, mut gap: i32) -> String {
    let grid_line_length = line_length / scale as f64;

    let grid_count: f64;
    if scale < 0.6 {
        gap = 80;
        grid_count = grid_line_length / gap as f64;
    } else {
        grid_count = grid_line_length / gap as f64;
    }

    let mut points = Vec::with_capacity((grid_count * grid_count) as usize);

    for i in 0..(grid_count as i32) * (grid_count as i32) {
        let x = (i % (grid_count as i32)) as f64 * gap as f64 - grid_line_length / 2.0;
        let y = (i / (grid_count as i32)) as f64 * gap as f64 - grid_line_length / 2.0;
        points.push((x, y));
    }

    let mut buf = String::with_capacity(points.len() * 8 + 2);
    buf.push('[');

    for (i, (x, y)) in points.iter().enumerate() {
        write!(buf, "({:.6},{:.6})", x, y).unwrap();
        if i < points.len() - 1 {
            buf.push(',');
        }
    }

    buf.push(']');
    buf
}
