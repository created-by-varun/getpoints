use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_points(grid_count: i32, grid_line_length: f64) -> String {
    let gap = grid_line_length / (grid_count as f64);
    let mut points = Vec::with_capacity((grid_count * grid_count) as usize);

    for i in 0..grid_count * grid_count {
        let x = (i % grid_count) as f64 * gap - grid_line_length / 2.0;
        let y = (i / grid_count) as f64 * gap - grid_line_length / 2.0;
        points.push((x, y));
    }

    let points_str = points
        .iter()
        .map(|(x, y)| format!("({},{})", x, y))
        .collect::<Vec<String>>()
        .join(",");

    format!("[{}]", points_str)
}
