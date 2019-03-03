use std::collections::VecDeque;

use arrayfire::Array;

use crate::RawType;

pub fn difference(
    arr1: &arrayfire::Array<crate::RawFtType>,
    arr2: &arrayfire::Array<crate::RawFtType>,
) -> arrayfire::Array<crate::RawType> {
    arrayfire::abs(&(arr1 - arr2))
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
            self.data.push_back(array);
        } else {
            self.data.push_back(array);
        }
    }
}


pub fn mean_image(arr: &VecDeque<arrayfire::Array<crate::RawType>>) -> Option<arrayfire::Array<crate::RawType>> {
    let mut sum: Option<Array<crate::RawType>> = None;
    for a in arr {
        if let Some(v) = sum {
            sum = Some(v + a);
        } else {
            sum = Some(a.clone());
        }
    }
    if let Some(s) = sum {
        Some(s / (arr.len() as f32))
    } else {
        None
    }
}