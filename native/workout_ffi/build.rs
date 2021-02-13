use dart_bindgen::{config::*, Codegen};

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("binding.h");

    let config = DynamicLibraryConfig {
        ios: DynamicLibraryCreationMode::Executable.into(),
        android: DynamicLibraryCreationMode::open("libworkout_ffi.so").into(),
        ..Default::default()
    };

    let codegen = Codegen::builder()
        .with_src_header("binding.h")
        .with_lib_name("libworkout")
        .with_config(config)
        .build()
        .unwrap();

    let _bindings = codegen.generate().unwrap();
    // TODO: enable once we got flutter plugin package
    /* bindings
    .write_to_file("../../packages/worker_ffi/lib/ffi.dart")
    .unwrap(); */
}
