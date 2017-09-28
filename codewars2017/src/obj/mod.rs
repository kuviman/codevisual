use ::*;

#[derive(Vertex, Debug, Copy, Clone)]
pub struct VertexData {
    pub a_v: Vec3<f32>,
    pub a_vt: Vec2<f32>,
    pub a_vn: Vec3<f32>,
}

fn parse_obj(source: &str) -> Vec<(String, Vec<VertexData>)> {
    let mut result = Vec::new();

    let mut current_name = String::from("__unnamed__");

    let mut v = Vec::new();
    let mut vn = Vec::new();
    let mut vt = Vec::new();
    let mut current_obj = Vec::new();
    for line in source.lines().chain(std::iter::once("o _")) {
        if line.starts_with("v ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            v.push(vec3(-x, z, y) / 2.0); //TODO: no negation, no division
        } else if line.starts_with("vn ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            vn.push(vec3(-x, z, y)); //TODO: no negation
        } else if line.starts_with("vt ") {
            let mut parts = line.split_whitespace();
            parts.next();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            vt.push(vec2(x, 1.0 - y));
        } else if line.starts_with("f ") {
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
                current_obj.push(cur[0]);
                current_obj.push(cur[i - 1]);
                current_obj.push(cur[i]);
            }
        } else if line.starts_with("o ") {
            if current_obj.len() != 0 {
                result.push((current_name, current_obj));
                current_obj = Vec::new();
            }
            current_name = String::from(&line[2..]);
        }
    }
    let mut min_x: f32 = 1e9;
    let mut max_x: f32 = -1e9;
    let mut min_y: f32 = 1e9;
    let mut max_y: f32 = -1e9;
    for &Vec3 { x, y, .. } in &v {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    let center = vec3(min_x + max_x, min_y + max_y, 0.0) / 2.0;
    let div = (max_y - min_y).max(max_x - min_x) / 2.0;
    for &mut (_, ref mut v) in &mut result {
        for v in v.iter_mut() {
            v.a_v = (v.a_v - center) / div;
        }
    }
    result
}

pub struct Model {
    pub geometry: ugli::VertexBuffer<VertexData>,
    pub texture: ugli::Texture2d,
}

pub struct ModelFuture {
    app: Rc<codevisual::Application>,
    texture: codevisual::TextureResourceFuture,
    geometry: codevisual::StringResourceFuture,
}

impl codevisual::Resource for Model {
    type Future = ModelFuture;
}

impl codevisual::Asset for Model {
    fn load(loader: &Rc<codevisual::ResourceLoader>, path: &str) -> ModelFuture {
        ModelFuture {
            app: (loader as &Rc<codevisual::Application>).clone(),
            texture: <ugli::Texture2d as codevisual::Asset>::load(
                loader, &format!("{}.png", path)),
            geometry: <String as codevisual::Asset>::load(
                loader, &format!("{}.obj", path)),
        }
    }
}

impl codevisual::ResourceFuture<Model> for ModelFuture {
    fn unwrap(self) -> Model {
        let mut texture = self.texture.unwrap();
        texture.gen_mipmaps();
        let geometry = {
            let mut geometry = Vec::new();
            for (_, mut data) in parse_obj(&self.geometry.unwrap()) {
                geometry.append(&mut data);
            }
            ugli::VertexBuffer::new_static(self.app.ugli_context(), geometry)
        };
        Model {
            texture,
            geometry,
        }
    }
}

pub struct RawModel {
    pub geometry: ugli::VertexBuffer<VertexData>,
}

pub struct RawModelFuture {
    app: Rc<codevisual::Application>,
    geometry: codevisual::StringResourceFuture,
}

impl codevisual::Resource for RawModel {
    type Future = RawModelFuture;
}

impl codevisual::Asset for RawModel {
    fn load(loader: &Rc<codevisual::ResourceLoader>, path: &str) -> RawModelFuture {
        RawModelFuture {
            app: (loader as &Rc<codevisual::Application>).clone(),
            geometry: <String as codevisual::Asset>::load(
                loader, &format!("{}.obj", path)),
        }
    }
}

impl codevisual::ResourceFuture<RawModel> for RawModelFuture {
    fn unwrap(self) -> RawModel {
        let geometry = {
            let mut geometry = Vec::new();
            for (_, mut data) in parse_obj(&self.geometry.unwrap()) {
                geometry.append(&mut data);
            }
            ugli::VertexBuffer::new_static(self.app.ugli_context(), geometry)
        };
        RawModel {
            geometry,
        }
    }
}

pub struct ModelParts {
    pub parts: Vec<(String, ugli::VertexBuffer<VertexData>)>,
    pub texture: ugli::Texture2d,
}

pub struct ModelPartsFuture {
    app: Rc<codevisual::Application>,
    texture: codevisual::TextureResourceFuture,
    geometry: codevisual::StringResourceFuture,
}

impl codevisual::Resource for ModelParts {
    type Future = ModelPartsFuture;
}

impl codevisual::Asset for ModelParts {
    fn load(loader: &Rc<codevisual::ResourceLoader>, path: &str) -> ModelPartsFuture {
        ModelPartsFuture {
            app: (loader as &Rc<codevisual::Application>).clone(),
            texture: <ugli::Texture2d as codevisual::Asset>::load(
                loader, &format!("{}.png", path)),
            geometry: <String as codevisual::Asset>::load(
                loader, &format!("{}.obj", path)),
        }
    }
}

impl codevisual::ResourceFuture<ModelParts> for ModelPartsFuture {
    fn unwrap(self) -> ModelParts {
        let app = self.app;
        ModelParts {
            texture: self.texture.unwrap(),
            parts: parse_obj(&self.geometry.unwrap()).into_iter().map(|(name, data)|
                (name, ugli::VertexBuffer::new_static(app.ugli_context(), data))).collect(),
        }
    }
}
