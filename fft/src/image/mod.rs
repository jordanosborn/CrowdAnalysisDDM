#[allow(dead_code)]
pub mod opencv {
    use libc::{c_char, c_int, c_void, size_t};

    extern "C" {
        fn start_capture(s: *const c_char) -> size_t;
        fn start_camera_capture() -> size_t;
        fn get_frame(stream_id: size_t) -> *mut c_void;
        fn mat_new() -> *mut CMat;
        fn mat_cols(cmat: *const CMat) -> c_int;
        fn mat_rows(cmat: *const CMat) -> c_int;
        fn mat_depth(cmat: *const CMat) -> c_int;
        fn mat_channels(cmat: *const CMat) -> c_int;
        fn mat_drop(cmat: *mut CMat);
    }

    #[derive(Clone, Debug)]
    pub enum CMat {}

    impl CMat {
        pub(crate) fn new() -> *mut CMat {
            unsafe { mat_new() }
        }
    }

    impl Drop for Mat {
        fn drop(&mut self) {
            unsafe {
                mat_drop(self.inner);
            }
        }
    }

    unsafe impl Send for CMat {}
    unsafe impl Send for Mat {}
    // impl Into<CMat> for Mat {
    //     fn into(self) -> CMat {
    //         unsafe { *self.inner }
    //     }
    // }

    #[derive(Debug)]
    pub struct Mat {
        /// Pointer to the actual C/C++ data structure
        pub(crate) inner: *mut CMat,

        /// Number of columns
        pub cols: c_int,

        /// Number of rows
        pub rows: c_int,

        /// Depth of this mat (it should be the type).
        pub depth: c_int,

        /// Channels of this mat
        pub channels: c_int,
    }

    impl Mat {
        #[inline]
        pub(crate) fn from_raw(raw: *mut CMat) -> Mat {
            Mat {
                inner: raw,
                rows: unsafe { mat_rows(raw) },
                cols: unsafe { mat_cols(raw) },
                depth: unsafe { mat_depth(raw) },
                channels: unsafe { mat_channels(raw) },
            }
        }
    }

    trait CString {
        fn c_string(&self) -> Vec<i8>;
    }

    impl CString for str {
        fn c_string(&self) -> Vec<i8> {
            self.as_bytes()
                .iter()
                .map(|&x| x as i8)
                .collect::<Vec<i8>>()
        }
    }

    pub fn start_capture_safe(s: &str) -> usize {
        let c_string = s.c_string();
        unsafe { start_capture(c_string.as_ptr()) }
    }

    pub fn start_camera_capture_safe() -> usize {
        unsafe { start_camera_capture() }
    }

    pub fn get_frame_safe(stream_id: usize) -> Vec<(u8, u8, u8)> {
        let _ptr: *const c_void;
        unsafe {
            _ptr = get_frame(stream_id);
        }
        vec![(1u8, 2u8, 3u8)]
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add_safe(1, 2), 3);
    // }
}
