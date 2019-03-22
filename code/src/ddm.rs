use std::collections::VecDeque;

use rayon::prelude::*;

use crate::operations;

pub fn ddm(
    accumulator: Option<VecDeque<arrayfire::Array<crate::RawType>>>,
    data: &VecDeque<arrayfire::Array<crate::RawFtType>>,
) -> Option<VecDeque<arrayfire::Array<crate::RawType>>> {
    match accumulator {
        Some(acc) => {
            let ft0 = &data[0];
            //At each t0 sum
            //Vec deque slice problem
            let mut data_slice = data.clone();
            data_slice.pop_front();
            Some(data_slice
                .par_iter()
                .zip(acc.par_iter())
                .map(|(i, a)| {
                    //TODO: WTF why does this work when loc is added below???!
                    arrayfire::imin_all(a);
                    a + operations::difference(i, ft0)
                })
                .collect::<VecDeque<arrayfire::Array<crate::RawType>>>())
        }
        None => {
            let ft0 = &data[0];
            Some(data.par_iter()
                .enumerate()
                .filter(|(i, _)| *i != 0usize)
                .map(|(_, x)| operations::difference(x, ft0))
                .collect::<VecDeque<arrayfire::Array<crate::RawType>>>()
            )
        }
    }
}
