use super::common::*;
use crate::native::opencv;

use super::multi_ddm::multi_ddm;

pub fn single_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    filename: Option<String>,
    output: Option<String>,
) -> Option<IndexedData> {
    let mut data_out = None;
    if let Some(id) = id {
        let (dim_x, dim_y) = opencv::dimension(id);
        let dimension = usize::max(dim_x, dim_y);
        let output = multi_ddm(
            Some(id),
            capacity,
            annuli_spacing,
            (Some(dimension), Some(dimension), Some(1)),
            None,
            None,
            filename,
            output,
        );
        if let Some(out) = output {
            if let Some(o) = out.get(&dimension) {
                data_out = Some(o.to_owned());
            }
        }
    } else {
        println!("Invalid arguments supplied!");
    }
    data_out
}
