extern {
    fn add(x: libc::c_int, y: libc::c_int) -> libc::c_int;
}

pub fn add_safe(x: i32, y: i32) -> i32 {
    let z: i32;
    unsafe {
        z = add(x, y);
    }
    z
}