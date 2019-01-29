extern "C" {
    fn start_capture(s: *const libc::c_char) -> libc::size_t;
    fn start_camera_capture() -> libc::size_t;
    // fn get_frame(stream_id: libc::size_t) -> *const libc::c_void;
}

trait CString {
    fn c_string(&self) -> Vec<i8>;
}

impl CString for str {
    fn c_string(&self) -> Vec<i8> {
        self.as_bytes().iter().map(|&x| {
            x as i8
        }).collect::<Vec<i8>>()
    }
}

pub fn start_capture_safe(s: &str) -> usize {
    let c_string = s.c_string();
    unsafe {
        start_capture(c_string.as_ptr())
    }
}

pub fn start_camera_capture_safe() -> usize {
    unsafe {
        start_camera_capture()
    }
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add_safe(1, 2), 3);
    // }
}
