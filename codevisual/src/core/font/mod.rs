use ::*;

#[derive(Debug)]
struct Vertex {
    a_pos: Vec2<f32>,
    a_vt: Vec2<f32>,
}

// TODO: derive should be in ugli-derive instead of codevisual-derive
impl ugli::Vertex for Vertex {
    fn walk_attributes<C: ugli::VertexAttributeConsumer>(&self, mut consumer: C) {
        consumer.consume("a_pos", &self.a_pos);
        consumer.consume("a_vt", &self.a_vt);
    }
}

pub struct Font {
    font: rusttype::Font<'static>,
    cache: RefCell<rusttype::gpu_cache::Cache<'static>>,
    cache_texture: RefCell<ugli::Texture2d>,
    geometry: RefCell<ugli::VertexBuffer<Vertex>>,
    material: Material,
}

const CACHE_SIZE: usize = 1024;

impl Font {
    pub fn default(context: &Rc<ugli::Context>) -> Rc<Font> {
        // TODO: create only one
        let data = include_bytes!("default-font.ttf") as &[u8];
        Rc::new(Font::new(context, data.to_owned()))
    }
    pub fn new(context: &Rc<ugli::Context>, data: Vec<u8>) -> Font {
        Font {
            font: rusttype::FontCollection::from_bytes(data).into_font().unwrap(),
            cache: RefCell::new(rusttype::gpu_cache::Cache::new(
                CACHE_SIZE as u32, CACHE_SIZE as u32, 0.1, 0.1)),
            cache_texture: RefCell::new(ugli::Texture2d::new_uninitialized(
                context, vec2(CACHE_SIZE, CACHE_SIZE))),
            geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(context, Vec::new())),
            material: Material::new(context, (), (), include_str!("shader.glsl")),
        }
    }
    pub fn measure_at(&self, text: &str, pos: Vec2<f32>, size: f32) -> Option<Rect<f32>> {
        let scale = rusttype::Scale { x: size, y: size };
        let pos = rusttype::Point { x: pos.x, y: pos.y };
        let mut result: Option<Rect<f32>> = None;
        for glyph in self.font.layout(text, scale, pos) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                if let Some(cur) = result {
                    result = Some(Rect::from_corners(
                        vec2(min(bb.min.x as f32, cur.bottom_left.x),
                             min(bb.min.y as f32, cur.bottom_left.y)),
                        vec2(max(bb.max.x as f32, cur.top_right.x),
                             max(bb.max.y as f32, cur.top_right.y))));
                } else {
                    result = Some(Rect::from_corners(vec2(bb.min.x as f32, bb.min.y as f32),
                                                     vec2(bb.max.x as f32, bb.max.y as f32)));
                }
            }
        }
        result
    }
    pub fn measure(&self, text: &str, size: f32) -> Option<Rect<f32>> {
        self.measure_at(text, vec2(0.0, 0.0), size)
    }
    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer,
                text: &str, pos: Vec2<f32>, size: f32, color: Color) {
        let scale = rusttype::Scale { x: size, y: size };
        let pos = rusttype::Point { x: pos.x, y: framebuffer.get_size().y as f32 - pos.y };

        let mut cache = self.cache.borrow_mut();
        let mut cache_texture = self.cache_texture.borrow_mut();

        let glyphs: Vec<_> = self.font.layout(text, scale, pos).collect();

        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.standalone()); // TODO: avoid cloning glyph?
        }

        cache.cache_queued(|rect, data| {
            let x = rect.min.x as usize;
            let y = rect.min.y as usize;
            let width = rect.width() as usize;
            let height = rect.height() as usize;

            // TODO: somehow without copying?
            let mut fixed_data = Vec::with_capacity(data.len() * 4);
            for byte in data {
                for _ in 0..3 {
                    fixed_data.push(0xff);
                }
                fixed_data.push(*byte);
            }

            unsafe {
                cache_texture.sub_image(vec2(x, y), vec2(width, height), &fixed_data);
            }
        }).unwrap();

        let mut geometry = self.geometry.borrow_mut();
        geometry.clear();
        for glyph in &glyphs {
            if let Some((texture_rect, rect)) = cache.rect_for(0, glyph).unwrap() {
                let x1 = rect.min.x as f32;
                let y1 = rect.min.y as f32;
                let x2 = rect.max.x as f32;
                let y2 = rect.max.y as f32;
                let u1 = texture_rect.min.x;
                let u2 = texture_rect.max.x;
                let v1 = texture_rect.min.y;
                let v2 = texture_rect.max.y;
                geometry.push(Vertex {
                    a_pos: vec2(x1, y1),
                    a_vt: vec2(u1, v1),
                });
                geometry.push(Vertex {
                    a_pos: vec2(x2, y1),
                    a_vt: vec2(u2, v1),
                });
                geometry.push(Vertex {
                    a_pos: vec2(x2, y2),
                    a_vt: vec2(u2, v2),
                });

                geometry.push(Vertex {
                    a_pos: vec2(x1, y1),
                    a_vt: vec2(u1, v1),
                });
                geometry.push(Vertex {
                    a_pos: vec2(x2, y2),
                    a_vt: vec2(u2, v2),
                });
                geometry.push(Vertex {
                    a_pos: vec2(x1, y2),
                    a_vt: vec2(u1, v2),
                });
            }
        }

        ugli::draw(framebuffer,
                   &self.material.ugli_program(),
                   ugli::DrawMode::Triangles,
                   &*geometry,
                   ugli::SingleUniform::new("u_color", color),
                   ugli::DrawParameters {
                       depth_func: None,
                       blend_mode: Some(ugli::BlendMode::Alpha),
                       ..default()
                   });
    }
}