#[cfg(feature = "generate-bindings")]
mod binding;

use safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(C)]
pub struct AppState {
    pub count: i64,
    pub msg: safer_ffi::String,
    pub err: safer_ffi::String,
}

#[ffi_export]
fn add_struct(a: i64, b: i64) -> AppState {
    let count = a + b;
    let err = if count < 5 {
        "No Error"
    } else {
        "Count should not go beyond 4"
    };
    AppState {
        count,
        msg: format!("Rust counted {} times.", count).into(),
        err: err.to_string().into(),
    }
}
