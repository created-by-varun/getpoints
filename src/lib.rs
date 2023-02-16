use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_points(grid_count: i32, grid_line_length: f64) -> String {
    let gap = grid_line_length / (grid_count as f64);
    let grid_count = grid_count as usize;
    let num_points = grid_count * grid_count;
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let x = (i % grid_count) as f64 * gap - grid_line_length / 2.0;
        let y = (i / grid_count) as f64 * gap - grid_line_length / 2.0;
        points.push([x, y]);
    }

    let mut buf = String::with_capacity(num_points * 10 + 2);
    buf.push('[');

    for (i, [x, y]) in points.iter().enumerate() {
        write!(buf, "({:.6},{:.6})", x, y).unwrap();

        if i < num_points - 1 {
            buf.push(',');
        }
    }

    buf.push(']');

    buf
}
