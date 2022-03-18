extern crate libloading as libc;
use std::ffi::CString;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoString {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}

fn call_go(go_name: GoString) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        #[cfg(target_os = "linux")]
        let libc = libc::Library::new("./libs/linux/libhello.so")?;
        #[cfg(target_os = "macos")]
        let libc = libc::Library::new("./libs/macos/libhello.so")?;
        #[cfg(target_os = "windows")]
        let libc = libc::Library::new("./libs/windows/libhello.dll")?;
        let func: libc::Symbol<unsafe extern "C" fn(GoString)> = libc.get(b"Hello")?;
        func(go_name);
        Ok(0)
    }
}

fn main() {
    let c_name = CString::new("Alex").unwrap();

    let go_str_ref = GoString {
        p: c_name.as_ptr(),
        n: c_name.as_bytes().len() as isize,
    };

    let _ = call_go(go_str_ref);
}
