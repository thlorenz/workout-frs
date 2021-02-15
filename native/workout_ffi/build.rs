#[cfg(not(feature = "generate-bindings"))]
fn main() {}

#[cfg(feature = "generate-bindings")]
fn main() {
    /*
    use std::process::Command;
    //
    // Config
    //
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_dir = format!(
        "{}/../../packages/{}",
        crate_dir,
        std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME")
    );
    let binding_h = format!("{}/binding.h", crate_dir);
    let lib_name = "libworkout_ffi";
    let dart_ffi = format!("{}/lib/ffi.dart", package_dir);

    //
    // dart-bindghen
    //

    //  dart-bindgen --input binding.h \
    //    --android 'libworkout_ffi.so' --ios executable --macos 'libworkout_ffi.dylib
    let result = Command::new("dart-bindgen")
        .args(&["--input", &binding_h])
        .args(&["--output", &dart_ffi])
        .args(&["--name", &lib_name])
        .args(&["--android", &format!("{}.so", &lib_name)])
        .args(&["--macos", &format!("{}.dylib", &lib_name)])
        .args(&["--ios", "executable"])
        .output()
        .expect("Failed to create dart-bindgen");

    assert!(result.status.success());
    */
}
