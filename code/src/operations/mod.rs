//use crate::native::opencv::{Image, GrayImage};

//TODO: needs to be an absolute difference
pub fn difference(
    arr1: &arrayfire::Array<crate::RawType>,
    arr2: &arrayfire::Array<crate::RawType>,
) -> arrayfire::Array<u8> {
    arr1 - arr2
}
