use ::*;


pub struct Material {
    inner: RefCell<codevisual::Material<ShaderLib, (), settings::ShaderDefines>>,
    settings: Rc<Settings>,
}

pub struct UgliProgramGuard<'a> {
    program: *const ugli::Program,
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> Deref for UgliProgramGuard<'a> {
    type Target = ugli::Program;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.program }
    }
}

impl Material {
    pub fn new(context: &Rc<ugli::Context>,
               settings: &Rc<Settings>,
               program_source: &str) -> Self {
        Self {
            inner: RefCell::new(codevisual::Material::new(
                context, (), settings.get_shader_defines(), program_source)),
            settings: settings.clone(),
        }
    }

    pub fn ugli_program(&self) -> UgliProgramGuard {
        self.inner.borrow_mut().defines = self.settings.get_shader_defines();
        let borrow = self.inner.borrow();
        let program = borrow.ugli_program();
        let program = { &*program } as *const _; // TODO: possible without unsafe?
        UgliProgramGuard {
            program,
            phantom_data: PhantomData,
        }
    }
}