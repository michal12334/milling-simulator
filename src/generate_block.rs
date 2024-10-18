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
            let tc = (
                x as f32 / ((resolution.0 - 1) as f32),
                z as f32 / ((resolution.2 - 1) as f32),
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
                Vertex::from_tuples(b, tc, top_normal),
                Vertex::from_tuples(a, tc, top_normal),
                Vertex::from_tuples(c, tc, top_normal),
                Vertex::from_tuples(d, tc, top_normal),
                Vertex::from_tuples(b, tc, top_normal),
                Vertex::from_tuples(c, tc, top_normal),
                Vertex::from_tuples(e, tc, bottom_normal),
                Vertex::from_tuples(f, tc, bottom_normal),
                Vertex::from_tuples(g, tc, bottom_normal),
                Vertex::from_tuples(f, tc, bottom_normal),
                Vertex::from_tuples(h, tc, bottom_normal),
                Vertex::from_tuples(g, tc, bottom_normal),
                Vertex::from_tuples(a, tc, front_normal),
                Vertex::from_tuples(b, tc, front_normal),
                Vertex::from_tuples(e, tc, front_normal),
                Vertex::from_tuples(f, tc, front_normal),
                Vertex::from_tuples(e, tc, front_normal),
                Vertex::from_tuples(b, tc, front_normal),
                Vertex::from_tuples(h, tc, left_normal),
                Vertex::from_tuples(f, tc, left_normal),
                Vertex::from_tuples(d, tc, left_normal),
                Vertex::from_tuples(b, tc, left_normal),
                Vertex::from_tuples(d, tc, left_normal),
                Vertex::from_tuples(f, tc, left_normal),
                Vertex::from_tuples(d, tc, back_normal),
                Vertex::from_tuples(c, tc, back_normal),
                Vertex::from_tuples(g, tc, back_normal),
                Vertex::from_tuples(g, tc, back_normal),
                Vertex::from_tuples(h, tc, back_normal),
                Vertex::from_tuples(d, tc, back_normal),
                Vertex::from_tuples(c, tc, right_normal),
                Vertex::from_tuples(a, tc, right_normal),
                Vertex::from_tuples(g, tc, right_normal),
                Vertex::from_tuples(e, tc, right_normal),
                Vertex::from_tuples(g, tc, right_normal),
                Vertex::from_tuples(a, tc, right_normal),
            ]
        })
        .collect()
}
