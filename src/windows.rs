extern crate kernel32;

fn locale_raw(out_buffer: &mut [u16], getter: unsafe extern "system" fn(*mut u16, i32) -> i32) -> usize {
    unsafe { getter(out_buffer as *mut _ as *mut u16, out_buffer.len() as i32) as usize }
}

fn system_locale_raw(out_buffer: &mut [u16]) -> usize {
    locale_raw(out_buffer, kernel32::GetSystemDefaultLocaleName)
}
fn user_locale_raw(out_buffer: &mut [u16]) -> usize {
    locale_raw(out_buffer, kernel32::GetUserDefaultLocaleName)
}

pub fn system() -> String {
    let mut locale_raw = [0u16; 10];
    let res = system_locale_raw(&mut locale_raw);
    String::from_utf16(&locale_raw[..res as usize - 1]).unwrap()
}

pub fn user() -> String {
    let mut locale_raw = [0u16; 10];
    let res = user_locale_raw(&mut locale_raw);
    String::from_utf16(&locale_raw[..res as usize - 1]).unwrap()
}
