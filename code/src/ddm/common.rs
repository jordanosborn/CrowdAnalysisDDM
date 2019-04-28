use rayon::prelude::*;
use std::collections::VecDeque;

use crate::operations;

pub fn ddm(
    accumulator: Option<VecDeque<arrayfire::Array<crate::RawType>>>,
    data: &VecDeque<arrayfire::Array<crate::RawFtType>>,
) -> Option<VecDeque<arrayfire::Array<crate::RawType>>> {
    match accumulator {
        Some(acc) => {
            let mut data_slice = data.clone();
            let ft0 = data_slice.pop_front().unwrap();
            Some(
                data_slice
                    .par_iter()
                    .zip(acc.par_iter())
                    .map(|(i, a)| {
                        //TODO: WTH why does this work when loc is added below???! panics at t0 = 47 ??????
                        //This works on mac mini
                        //gpu issues?
                        arrayfire::imin_all(a);
                        a + operations::difference(i, &ft0)
                    })
                    .collect::<VecDeque<arrayfire::Array<crate::RawType>>>(),
            )
        }
        None => {
            let mut data_slice = data.clone();
            let ft0 = data_slice.pop_front().unwrap();
            Some(
                data_slice
                    .par_iter()
                    .enumerate()
                    .map(|(_, x)| operations::difference(x, &ft0))
                    .collect::<VecDeque<arrayfire::Array<crate::RawType>>>(),
            )
        }
    }
}

pub enum Signal {
    KILL,
}

pub type IndexedData = (
    Vec<crate::RawType>,
    Vec<Vec<(crate::RawType, crate::RawType)>>,
);
