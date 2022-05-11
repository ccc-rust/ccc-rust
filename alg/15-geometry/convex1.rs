// https://github.com/TheAlgorithms/Rust/blob/master/src/general/convex_hull.rs

use std::cmp::Ordering::Equal;

fn sort_by_min_angle(pts: &[(f64, f64)], min: &(f64, f64)) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64, (f64, f64))> = pts
        .iter()
        .map(|x| {
            (
                ((x.1 - min.1) as f64).atan2((x.0 - min.0) as f64),
                // angle
                ((x.1 - min.1) as f64).hypot((x.0 - min.0) as f64),
                // distance (we want the closest to be first)
                *x,
            )
        })
        .collect();
    points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    points.into_iter().map(|x| x.2).collect()
}

// calculates the z coordinate of the vector product of vectors ab and ac
fn calc_z_coord_vector_product(a: &(f64, f64), b: &(f64, f64), c: &(f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)
}

/*
    If three points are aligned and are part of the convex hull then the three are kept.
    If one doesn't want to keep those points, it is easy to iterate the answer and remove them.
    The first point is the one with the lowest y-coordinate and the lowest x-coordinate.
    Points are then given counter-clockwise, and the closest one is given first if needed.
*/
pub fn convex_hull_graham(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    if pts.is_empty() {
        return vec![];
    }

    let mut stack: Vec<(f64, f64)> = vec![];
    let min = pts
        .iter()
        .min_by(|a, b| {
            let ord = a.1.partial_cmp(&b.1).unwrap_or(Equal);
            match ord {
                Equal => a.0.partial_cmp(&b.0).unwrap_or(Equal),
                o => o,
            }
        })
        .unwrap();
    let points = sort_by_min_angle(pts, &min);

    if points.len() <= 3 {
        return points;
    }

    for point in points {
        while stack.len() > 1
            && calc_z_coord_vector_product(&stack[stack.len() - 2], &stack[stack.len() - 1], &point)
                < 0.
        {
            stack.pop();
        }
        stack.push(point);
    }

    stack
}

fn main() {
      let list = vec![
        (4.4, 14.),
        (6.7, 15.25),
        (6.9, 12.8),
        (2.1, 11.1),
        (9.5, 14.9),
        (13.2, 11.9),
        (10.3, 12.3),
        (6.8, 9.5),
        (3.3, 7.7),
        (0.6, 5.1),
        (5.3, 2.4),
        (8.45, 4.7),
        (11.5, 9.6),
        (13.8, 7.3),
        (12.9, 3.1),
        (11., 1.1),
    ];

    println!("convex_hull_graham()={:?}", convex_hull_graham(&list));
}