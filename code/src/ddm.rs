use std::collections::VecDeque;

use rayon::prelude::*;

use crate::operations;

pub fn ddm_0(data: &VecDeque<arrayfire::Array<crate::RawFtType>>) -> VecDeque<arrayfire::Array<crate::RawType>> {
    let ft0 = &data[0];
    data.par_iter().enumerate().filter(|(i, _)| {
        *i != 0usize
    }).map(|(_, x)| {
        operations::difference(x, ft0)
    }).collect::<VecDeque<arrayfire::Array<crate::RawType>>>()
}

pub fn ddm(acc: VecDeque<arrayfire::Array<crate::RawType>>, data: &VecDeque<arrayfire::Array<crate::RawFtType>>) -> VecDeque<arrayfire::Array<crate::RawType>> {
    let ft0 = &data[0];
//At each t0 sum and average at each t0
    let intensities = data.par_iter().enumerate().filter(|(i, _)| {
        *i != 0usize
    }).map(|(_, x)| {
        operations::difference(x, ft0)
    }).collect::<VecDeque<arrayfire::Array<crate::RawType>>>();
    intensities
//    intensities.iter().zip(acc.iter()).map(|(intensity, acc)| {
//        acc + intensity
//    }).collect::<VecDeque<arrayfire::Array<crate::RawType>>>()
}

