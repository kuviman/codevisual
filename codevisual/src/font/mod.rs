use ::*;

#[derive(Vertex, Debug)]
struct Vertex {
    a_pos: Vec2<f32>,
    a_vt: Vec2<f32>,
}

pub struct Font {
    font: rusttype::Font<'static>,
    cache: RefCell<rusttype::gpu_cache::Cache<'static>>,
    cache_texture: RefCell<ugli::Texture>,
    geometry: RefCell<ugli::VertexBuffer<Vertex>>,
    program: ugli::Program,
}

const CACHE_SIZE: usize = 1024;

impl Font {
    pub fn new(app: &Rc<App>, data: Vec<u8>) -> Result<Font, Error> {
        Self::new_with(app.ugli_context(), app.shader_lib(), data)
    }
    pub(crate) fn new_with(
        context: &Rc<ugli::Context>,
        shader_lib: &ShaderLib,
        data: Vec<u8>,
    ) -> Result<Font, Error> {
        Ok(Font {
            font: match rusttype::FontCollection::from_bytes(data).into_font() {
                Some(font) => font,
                None => bail!("Failed to load font"),
            },
            cache: RefCell::new(rusttype::gpu_cache::Cache::new(
                CACHE_SIZE as u32,
                CACHE_SIZE as u32,
                0.1,
                0.1,
            )),
            cache_texture: RefCell::new(ugli::Texture2d::new_uninitialized(
                context,
                vec2(CACHE_SIZE, CACHE_SIZE),
            )),
            geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(context, Vec::new())),
            program: shader_lib.compile(include_str!("shader.glsl")).unwrap(),
        })
    }
    pub fn measure_at(&self, text: &str, pos: Vec2<f32>, size: f32) -> Option<Rect<f32>> {
        let scale = rusttype::Scale { x: size, y: size };
        let pos = rusttype::Point { x: pos.x, y: pos.y };
        let mut result: Option<Rect<f32>> = None;
        for glyph in self.font.layout(text, scale, pos) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                if let Some(cur) = result {
                    result = Some(Rect::from_corners(
                        vec2(
                            min(bb.min.x as f32, cur.bottom_left.x),
                            min(bb.min.y as f32, cur.bottom_left.y),
                        ),
                        vec2(
                            max(bb.max.x as f32, cur.top_right.x),
                            max(bb.max.y as f32, cur.top_right.y),
                        ),
                    ));
                } else {
                    result = Some(Rect::from_corners(
                        vec2(bb.min.x as f32, bb.min.y as f32),
                        vec2(bb.max.x as f32, bb.max.y as f32),
                    ));
                }
            }
        }
        result
    }
    pub fn measure(&self, text: &str, size: f32) -> Option<Rect<f32>> {
        self.measure_at(text, vec2(0.0, 0.0), size)
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        text: &str,
        pos: Vec2<f32>,
        size: f32,
        color: Color,
    ) {
        let scale = rusttype::Scale { x: size, y: size };
        let pos = rusttype::Point {
            x: pos.x,
            y: framebuffer.get_size().y as f32 - pos.y,
        };

        let mut cache = self.cache.borrow_mut();
        let mut cache_texture = self.cache_texture.borrow_mut();

        let glyphs: Vec<_> = self.font.layout(text, scale, pos).collect();

        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.standalone()); // TODO: avoid cloning glyph?
        }

        cache
            .cache_queued(|rect, data| {
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
            })
            .unwrap();

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

        let framebuffer_size = framebuffer.get_size();

        ugli::draw(
            framebuffer,
            &self.program,
            ugli::DrawMode::Triangles,
            &*geometry,
            uniforms! {
                u_color: color,
                u_cache_texture: &*cache_texture,
                u_framebuffer_size: framebuffer_size,
            },
            ugli::DrawParameters {
                depth_func: None,
                blend_mode: Some(ugli::BlendMode::Alpha),
                ..default()
            },
        );
    }
    pub fn draw_aligned(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        text: &str,
        pos: Vec2<f32>,
        align: f32,
        size: f32,
        color: Color,
    ) {
        if let Some(rect) = self.measure(text, size) {
            self.draw(
                framebuffer,
                text,
                vec2(pos.x - rect.width() * align, pos.y),
                size,
                color,
            );
        }
    }
}
