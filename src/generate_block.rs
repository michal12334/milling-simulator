use crate::vertex::Vertex;

pub fn generate_block(size: (f32, f32, f32), resolution: (u32, u32)) -> Vec<Vertex> {
    let single_size = (size.0 / (resolution.0 as f32), size.2 / (resolution.1 as f32));
    (0..resolution.0)
        .flat_map(|x| (0..resolution.1).map(move |z| (x, z)))
        .flat_map(|(x, z)| {
            let tc = (x as f32 / ((resolution.0 - 1) as f32), z as f32 / ((resolution.1 - 1) as f32));
            let a = ((x as f32 / resolution.0 as f32 - 0.5) * size.0, size.1, (z as f32 / resolution.1 as f32 - 0.5) * size.2);
            let b = (a.0 + single_size.0, a.1, a.2);
            let c = (a.0, a.1, a.2 + single_size.1);
            let d = (a.0 + single_size.0, a.1, a.2 + single_size.1);

            let e = (a.0, -a.1, a.2);
            let f = (b.0, -b.1, b.2);
            let g = (c.0, -c.1, c.2);
            let h = (d.0, -d.1, d.2);
            [
                Vertex::from_tuples(a, tc), Vertex::from_tuples(b, tc), Vertex::from_tuples(c, tc),
                Vertex::from_tuples(b, tc), Vertex::from_tuples(d, tc), Vertex::from_tuples(c, tc),
            ]
        })
        .collect()
}
