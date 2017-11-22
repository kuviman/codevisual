use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: i32,
    minor: i32,
}

#[derive(Debug, Clone)]
pub struct ContextAttributes {
    pub alpha: bool,
    pub depth: bool,
    pub stencil: bool,
    pub antialias: bool,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub prefer_low_power_to_high_performance: bool,
    pub fail_if_major_performance_caveat: bool,
    pub version: Version,
    pub enable_extensions_by_default: bool,
}

impl ContextAttributes {
    fn from(attr: &EmscriptenWebGLContextAttributes) -> ContextAttributes {
        ContextAttributes {
            alpha: attr.alpha == EM_TRUE,
            depth: attr.depth == EM_TRUE,
            stencil: attr.stencil == EM_TRUE,
            antialias: attr.antialias == EM_TRUE,
            premultiplied_alpha: attr.premultipliedAlpha == EM_TRUE,
            preserve_drawing_buffer: attr.preserveDrawingBuffer == EM_TRUE,
            prefer_low_power_to_high_performance: attr.preferLowPowerToHighPerformance == EM_TRUE,
            fail_if_major_performance_caveat: attr.failIfMajorPerformanceCaveat == EM_TRUE,
            version: Version {
                major: attr.majorVersion as i32,
                minor: attr.minorVersion as i32,
            },
            enable_extensions_by_default: attr.enableExtensionsByDefault == EM_TRUE,
        }
    }
    fn into(&self) -> EmscriptenWebGLContextAttributes {
        EmscriptenWebGLContextAttributes {
            alpha: if self.alpha { EM_TRUE } else { EM_FALSE },
            depth: if self.depth { EM_TRUE } else { EM_FALSE },
            stencil: if self.stencil { EM_TRUE } else { EM_FALSE },
            antialias: if self.antialias { EM_TRUE } else { EM_FALSE },
            premultipliedAlpha: if self.premultiplied_alpha { EM_TRUE } else { EM_FALSE },
            preserveDrawingBuffer: if self.preserve_drawing_buffer { EM_TRUE } else { EM_FALSE },
            preferLowPowerToHighPerformance: if self.prefer_low_power_to_high_performance { EM_TRUE } else { EM_FALSE },
            failIfMajorPerformanceCaveat: if self.fail_if_major_performance_caveat { EM_TRUE } else { EM_FALSE },
            majorVersion: self.version.major as c_int,
            minorVersion: self.version.minor as c_int,
            enableExtensionsByDefault: if self.enable_extensions_by_default { EM_TRUE } else { EM_FALSE },
        }
    }
}

impl Default for ContextAttributes {
    fn default() -> Self {
        let mut attr: EmscriptenWebGLContextAttributes = unsafe { mem::uninitialized() };
        unsafe { emscripten_webgl_init_context_attributes(&mut attr); }
        ContextAttributes::from(&attr)
    }
}

#[derive(Debug)]
pub struct Context(EMSCRIPTEN_WEBGL_CONTEXT_HANDLE);

impl Context {
    pub fn create(target: Selector, attr: &ContextAttributes) -> HtmlResult<Context> {
        let attr = attr.into();
        let result = unsafe {
            emscripten_webgl_create_context(selector_as_ptr!(target), &attr)
        };
        if result > 0 {
            Ok(Context(result))
        } else {
            Err(parse_html_result(result as EMSCRIPTEN_RESULT).unwrap())
        }
    }

    pub fn make_current(&self) -> HtmlResult<()> {
        let result = unsafe {
            emscripten_webgl_make_context_current(self.0)
        };
        match parse_html_result(result) {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }
}