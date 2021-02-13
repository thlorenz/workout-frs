// TODO: just here to test flutter integration
#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    workout::add(a, b)
}
