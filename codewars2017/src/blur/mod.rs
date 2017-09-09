use ::*;

pub fn gauss(context: &Rc<ugli::Context>, texture: &ugli::Texture2d) -> ugli::Texture2d {
    let mut result = ugli::Texture2d::new_uninitialized(context, texture.get_size());
    let material: codevisual::Material = codevisual::Material::new(
        context, (), (), include_str!("shader.glsl"));
    ugli::draw(
        &mut ugli::Framebuffer::new_color(context, ugli::ColorAttachment::Texture(&mut result)),
        &material.ugli_program(),
        ugli::Quad::DRAW_MODE,
        &ugli::plain(&ugli::quad(context).slice(..)),
        uniforms!(texture: texture),
        &ugli::DrawParameters {
            depth_test: ugli::DepthTest::Off,
            blend_mode: ugli::BlendMode::Off,
            ..Default::default()
        }
    );
    result
}