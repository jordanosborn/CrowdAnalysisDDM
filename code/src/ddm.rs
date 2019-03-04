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

pub fn ddm(accumulator: VecDeque<arrayfire::Array<crate::RawType>>, data: &VecDeque<arrayfire::Array<crate::RawFtType>>) -> VecDeque<arrayfire::Array<crate::RawType>> {
    let ft0 = &data[0];
//At each t0 sum and average at each t0
    let intensities = data.par_iter().enumerate().filter(|(i, _)| {
        *i != 0usize
    }).map(|(_, x)| {
        operations::difference(x, ft0)
    }).collect::<Vec<arrayfire::Array<crate::RawType>>>();
    intensities.par_iter().zip(accumulator.par_iter()).map(|(intensity, acc)| {
        // What on earth is going on?????!!!
        // Why does it now not crash when the imin_all statement is inserted below
        // Something to do with optimisation???
        arrayfire::imin_all(acc);
        intensity + acc
    }).collect::<VecDeque<arrayfire::Array<crate::RawType>>>()
}

