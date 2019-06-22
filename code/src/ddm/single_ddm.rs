use crate::native::opencv;

use super::multi_ddm::{MultiDdmData, multi_ddm};

pub fn single_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    filename: Option<String>,
    output: Option<String>,
) -> (Option<String>, Option<MultiDdmData>) {
    if let Some(id) = id {
        let (dim_x, dim_y) = opencv::dimension(id);
        let dimension = usize::max(dim_x, dim_y);
        multi_ddm(
            Some(id),
            capacity,
            annuli_spacing,
            (Some(dimension), Some(dimension), Some(1)),
            None,
            filename,
            output,
        )
    } else {
        println!("Invalid arguments supplied!");
        (None, None)
    }
}
