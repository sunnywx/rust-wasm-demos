#[no_mangle]
/*
 fixme: no_mangle attr is required
*/
pub extern "C" fn add_one(x: u32) -> u32 {
    return x + 1;
}
