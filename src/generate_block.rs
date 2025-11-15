use crate::vertex::Vertex;

pub fn generate_block(size: (f32, f32, f32), resolution: (u32, u32, u32)) -> Vec<Vertex> {
    let top_normal = 0;
    let bottom_normal = 1;
    let right_normal = 2;
    let left_normal = 3;
    let front_normal = 4;
    let back_normal = 5;

    let single_size = (
        size.0 / (resolution.0 as f32),
        size.2 / (resolution.2 as f32),
    );
    (0..resolution.0)
        .flat_map(|x| (0..resolution.2).map(move |z| (x, z)))
        .flat_map(|(x, z)| {
            let top_coordinates = (
                z as f32 / ((resolution.2 - 1) as f32),
                x as f32 / ((resolution.0 - 1) as f32),
            );
            let front_bottom_coordinates = (
                (z - if z > 0 { 1 } else { 0 }) as f32 / ((resolution.2 - 1) as f32),
                x as f32 / ((resolution.0 - 1) as f32),
            );
            let back_bottom_coordinates = (
                (z + if z < resolution.2 - 1 { 1 } else { 0 }) as f32 / ((resolution.2 - 1) as f32),
                x as f32 / ((resolution.0 - 1) as f32),
            );
            let right_bottom_coordinates = (
                z as f32 / ((resolution.2 - 1) as f32),
                (x - if x > 0 { 1 } else { 0 }) as f32 / ((resolution.0 - 1) as f32),
            );
            let left_bottom_coordinates = (
                z as f32 / ((resolution.2 - 1) as f32),
                (x + if x < resolution.0 - 1 { 1 } else { 0 }) as f32 / ((resolution.0 - 1) as f32),
            );
            let a = (
                (x as f32 / resolution.0 as f32 - 0.5) * size.0,
                size.1 / 2.0,
                (z as f32 / resolution.2 as f32 - 0.5) * size.2,
            );
            let b = (a.0 + single_size.0, a.1, a.2);
            let c = (a.0, a.1, a.2 + single_size.1);
            let d = (a.0 + single_size.0, a.1, a.2 + single_size.1);

            let e = (a.0, -a.1, a.2);
            let f = (b.0, -b.1, b.2);
            let g = (c.0, -c.1, c.2);
            let h = (d.0, -d.1, d.2);
            [
                Vertex::from_tuples(b, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(a, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(c, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(d, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(b, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(c, top_coordinates, top_coordinates, top_normal),
                Vertex::from_tuples(e, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(f, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(g, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(f, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(h, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(g, top_coordinates, top_coordinates, bottom_normal),
                Vertex::from_tuples(a, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(b, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(e, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(f, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(e, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(b, top_coordinates, front_bottom_coordinates, front_normal),
                Vertex::from_tuples(h, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(f, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(d, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(b, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(d, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(f, top_coordinates, left_bottom_coordinates, left_normal),
                Vertex::from_tuples(d, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(c, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(g, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(g, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(h, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(d, top_coordinates, back_bottom_coordinates, back_normal),
                Vertex::from_tuples(c, top_coordinates, right_bottom_coordinates, right_normal),
                Vertex::from_tuples(a, top_coordinates, right_bottom_coordinates, right_normal),
                Vertex::from_tuples(g, top_coordinates, right_bottom_coordinates, right_normal),
                Vertex::from_tuples(e, top_coordinates, right_bottom_coordinates, right_normal),
                Vertex::from_tuples(g, top_coordinates, right_bottom_coordinates, right_normal),
                Vertex::from_tuples(a, top_coordinates, right_bottom_coordinates, right_normal),
            ]
        })
        .collect()
}
