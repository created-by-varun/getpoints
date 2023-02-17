use std::fmt::Write;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_points( line_length: f64, scale: i32, gap: i32) -> String {
    // Calculate the gap between each point in the grid
    //let gap = grid_line_length / (grid_count as f64);
    let grid_line_length = line_length/ scale;
    let grid_count = grid_line_length/gap;

    if scale< 0.6 {
        gap = 80;
        grid_count = grid_line_length/ gap;
    }

    // Create a vector to store the points in
    let mut points = Vec::with_capacity((grid_count * grid_count) as usize);

    // Loop through each point in the grid
    for i in 0..grid_count * grid_count {
        // Calculate the x and y coordinates for the point
        let x = (i % grid_count) as f64 * gap - grid_line_length / 2.0;
        let y = (i / grid_count) as f64 * gap - grid_line_length / 2.0;

        // Add the point to the vector
        points.push((x, y));
    }

    // Create a string buffer to store the points in
    let mut buf = String::with_capacity(points.len() * 8 + 2);

    // Start the points with an opening square bracket
    buf.push('[');

    // Loop through each point and add it to the buffer
    for (i, (x, y)) in points.iter().enumerate() {
        // Add the point to the buffer
        write!(buf, "({:.6},{:.6})", x, y).unwrap();

        // Add a comma after the point unless it's the last point
        if i < points.len() - 1 {
            buf.push(',');
        }
    }

    // Add a closing square bracket to the end of the points
    buf.push(']');

    // Return the points as a string
    buf
}
