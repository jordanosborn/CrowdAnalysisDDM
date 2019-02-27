use arrayfire::Array;
use std::collections::VecDeque;

//TODO: needs to be an absolute difference
pub fn difference(
    arr1: &arrayfire::Array<crate::RawType>,
    arr2: &arrayfire::Array<crate::RawType>,
) -> arrayfire::Array<crate::RawType> {
    arr1 - arr2
}



pub struct Data {
    pub time_delta: f64,
    pub data: VecDeque<Array<crate::RawFtType>>,
    pub capacity: Option<usize>
}

impl Data {
    pub fn new(fps: usize, capacity: Option<usize>) -> Data {
        if let Some(size) = capacity {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::with_capacity(size),
                capacity: Some(size)
            }
        } else {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::new(),
                capacity: None
            }
        }
    }
    pub fn push(&mut self, array: Array<crate::RawFtType>) {
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
