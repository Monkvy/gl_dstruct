
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


use std::{rc::Rc, ops::Deref};

#[derive(Clone)]
pub struct Gl {
    inner: Rc<gl::Gl>
}

impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn load_with<F>(loadfn: F) -> Gl
where F: FnMut(&'static str) -> *const gl::types::GLvoid {
    Gl {
        inner: Rc::new(gl::Gl::load_with(loadfn))
    }
}
