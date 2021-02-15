mod fluid;

#[repr(C)]
pub struct AppState {
    pub count: i64,
    pub msg: fluid::Vec<u8>,
    pub err: fluid::Vec<u8>,
}

#[no_mangle]
pub extern "C" fn add_struct(a: i64, b: i64) -> AppState {
    let count = a + b;
    let msg = format!("Rust counted {} times", count);
    let err = if count > 4 {
        format!("{} > max count which is 4", count)
    } else {
        "".to_string()
    };
    AppState {
        count,
        msg: msg.into(),
        err: err.into(),
    }
}
