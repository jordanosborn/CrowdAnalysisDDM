extern "C" {
    //fn _start_capture(s: *const libc::c_char) -> libc::size_t;
    fn start_camera_capture() -> libc::size_t;
    fn show_next(stream_id: libc::size_t);
    fn get_frame(stream_id: libc::size_t) -> *const libc::c_void;
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

// pub fn start_capture(s: &str) -> usize {
//     let c_string = s.c_string();
//     unsafe {
//         _start_capture(c_string.as_ptr())
//     }
// }

pub fn start_camera_capture_safe() -> usize {
    unsafe {
        start_camera_capture()
    }
}

pub fn show_next_safe(stream_id: usize) {
    unsafe {
        show_next(stream_id);
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
