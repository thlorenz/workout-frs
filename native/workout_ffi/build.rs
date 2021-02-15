use std::{
    fs::File,
    io::Write,
    process::{exit, Command, Output},
};

fn write_cmd_output(cmd: &str, output: Output, file_path: &str) {
    if !output.status.success() {
        format!(
            "{} failed: \n{}",
            cmd,
            std::str::from_utf8(&output.stderr).unwrap()
        );
        exit(1);
    }
    let mut file = File::create(&file_path).expect("Unable to open binding.h");
    file.write_all(&output.stdout)
        .expect("Unable to write to binding.h");
}

fn main() {
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
    let fluid_dart = format!("{}/lib/fluid.dart", package_dir);

    //
    // cbindgen
    //

    // cbindgen ./src/lib.rs -c cbindgen.toml | grep -v \#include | uniq
    let output = Command::new("cbindgen")
        .arg(format!("{}/src/lib.rs", crate_dir))
        .arg("-c")
        .arg(format!("{}/cbindgen.toml", crate_dir))
        .output()
        .expect("Failed to create binding.h");

    // TODO: remove unneeded includes from output if necessary,
    // as in: `grep -v \#include | uniq`
    write_cmd_output("bindgen", output, &binding_h);

    //
    // dart-bindgen
    //

    //  dart-bindgen --input binding.h \
    //    --android 'libworkout_ffi.so' --ios executable --macos 'libworkout_ffi.dylib
    let result = Command::new("dart-bindgen")
        .args(&["--input", &binding_h])
        .args(&["--output", &fluid_dart])
        .args(&["--name", &lib_name])
        .args(&["--android", &format!("{}.so", &lib_name)])
        .args(&["--macos", &format!("{}.dylib", &lib_name)])
        .args(&["--ios", "executable"])
        .output()
        .expect("Failed to create dart-bindgen");

    assert!(result.status.success());
}
