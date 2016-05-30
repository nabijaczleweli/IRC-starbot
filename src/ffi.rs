
fn with_utf8_ptr<T, F>(value: &str, callback: F) -> T
    where F: Fn(*const i8) -> T
{
    let converted = CString::new(value).unwrap();
    let char_ptr = converted.as_ptr();
    unsafe {
        let i8_ptr = mem::transmute::<*const libc::c_char, *const i8>(char_ptr);
        callback(i8_ptr)
    }
}
