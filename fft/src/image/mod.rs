extern "C" {
    fn add(x: libc::c_int, y: libc::c_int) -> libc::c_int;
}

pub fn add_safe(x: i32, y: i32) -> i32 {
    let z: i32;
    unsafe {
        z = add(x, y);
    }
    z
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_safe(1, 2), 3);
    }
}
