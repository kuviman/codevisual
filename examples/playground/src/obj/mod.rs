use ::*;

#[derive(Vertex, Debug, Copy, Clone)]
pub struct VertexData {
    pub a_v: Vec3<f32>,
    pub a_vt: Vec2<f32>,
    pub a_vn: Vec3<f32>,
}

pub type Geometry = ugli::VertexBuffer<VertexData>;

pub fn parse(app: &codevisual::Application, source: &str) -> Geometry {
    let mut v = Vec::new();
    let mut vn = Vec::new();
    let mut vt = Vec::new();
    let mut data = Vec::new();
    for line in source.lines() {
        let line: &str = line;
        if line.starts_with("v ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            v.push(vec3(x, z, y));
        } else if line.starts_with("vn") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            vn.push(vec3(x, z, y));

        } else if line.starts_with("vt") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            vt.push(vec2(x, 1.0 - y));
        } else if line.starts_with("f") {
            let mut parts = line.split_whitespace();
            parts.next();
            let to_vertex = |s: &str| {
                let mut parts = s.split("/");
                let i_v: usize = parts.next().unwrap().parse().unwrap();
                let i_vt: usize = parts.next().unwrap().parse().unwrap();
                let i_vn: usize = parts.next().unwrap().parse().unwrap();
                VertexData {
                    a_v: v[i_v - 1],
                    a_vn: vn[i_vn - 1],
                    a_vt: vt[i_vt - 1],
                }
            };
            let mut cur = Vec::new();
            while let Some(s) = parts.next() {
                cur.push(to_vertex(s));
            }
            for i in 2..cur.len() {
                data.push(cur[0]);
                data.push(cur[i - 1]);
                data.push(cur[i]);
            }
        }
    }
    ugli::VertexBuffer::new(app.get_window().ugli_context(), data)
}