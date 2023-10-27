use std::{env, fs::File, path::Path};
use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator, DebugStructGenerator};


fn main() {
    let mut file = File::create(&Path::new(
        &env::var("OUT_DIR").unwrap()).join("bindings.rs")
    ).unwrap();

    let gl_version = env::var("GL_VERSION").expect("'GL_VERSION' environment variable not specified");
    let version: Vec<u8> = gl_version.split('.').collect::<Vec<&str>>().iter().map(|s| 
        s.parse::<u8>().expect("Invalid version specified in 'GL_VERSION' environment variable")
    ).collect();

    if version.len() != 2 {
        panic!("Invalid format of the 'GL_VERSION' environment variable")
    }

    let registry = Registry::new(Api::Gl, (version[0], version[1]), Profile::Core, Fallbacks::All, [/*"GL_NV_command_list", */]);
 
    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry.write_bindings(DebugStructGenerator, &mut file).unwrap();
    } else {
        registry.write_bindings(StructGenerator, &mut file).unwrap();
    }
}
