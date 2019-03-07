use std::collections::VecDeque;

use arrayfire::Array;

pub fn difference(
    arr1: &arrayfire::Array<crate::RawFtType>,
    arr2: &arrayfire::Array<crate::RawFtType>,
) -> arrayfire::Array<crate::RawType> {
    let abs = arrayfire::abs(&(arr1 - arr2));
    arrayfire::mul(&abs, &abs, true)
}

pub fn radial_average(
    arr: VecDeque<arrayfire::Array<crate::RawType>>,
) -> VecDeque<arrayfire::Array<crate::RawType>> {
    //TODO: Finish this function! should return 1D array I(q) for each tau
    arr
}

pub struct Data<T: arrayfire::HasAfEnum> {
    pub time_delta: f64,
    pub data: VecDeque<Array<T>>,
    pub capacity: Option<usize>,
}

impl<T: arrayfire::HasAfEnum> Data<T> {
    pub fn new(fps: usize, capacity: Option<usize>) -> Data<T> {
        if let Some(size) = capacity {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::with_capacity(size),
                capacity: Some(size),
            }
        } else {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::new(),
                capacity: None,
            }
        }
    }
    pub fn push(&mut self, array: Array<T>) {
        if let Some(capacity) = self.capacity {
            if self.data.len() == capacity {
                self.data.pop_front();
            }
        }
        self.data.push_back(array);
    }
}

pub fn mean_image(
    arr: &VecDeque<arrayfire::Array<crate::RawType>>,
) -> Option<arrayfire::Array<crate::RawType>> {
    if arr.is_empty() {
        let dims = arr[0].dims();
        Some(
            arr.iter().fold(
                arrayfire::Array::new_empty(dims),
                |acc: Array<crate::RawType>, x| acc + x,
            ) / arr.len() as f32,
        )
    } else {
        None
    }
}


pub fn create_annulus(dimension: u64, radius: u64, thickness: u64) -> arrayfire::Array<crate::RawType> {
    let mut annulus = vec![0f32;(dimension * dimension) as usize];
    let radius2 = radius * radius;
    let radius_plus_dr2 = (radius + thickness) * (radius + thickness);
    for i in 0..(dimension * dimension) {
        let x = i % dimension;
        let y = i / dimension;
        let r2 = (x - dimension / 2) * (x - dimension / 2) + (y - dimension / 2) * (y - dimension / 2);
        if  radius2 <= r2 && r2 <= radius_plus_dr2 {
            annulus[i as usize] = 1.0;
        }
    }
    let arr = Array::new(annulus.as_slice(), arrayfire::Dim4::new(&[dimension, dimension, 1, 1]));
    let divisor = arrayfire::sum_all(&arr).0 as f32;
    arr / divisor
}
