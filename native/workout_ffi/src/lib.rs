// TODO: just here to test flutter integration
#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    workout::add(a, b)
}

// Structs have to live in the same crate as otherwise cbindgen
// doesn't find them.
#[repr(C)]
pub struct AppState {
    pub count: i64,
}

#[no_mangle]
pub extern "C" fn add_struct(a: i64, b: i64) -> AppState {
    AppState { count: a + b }
}
