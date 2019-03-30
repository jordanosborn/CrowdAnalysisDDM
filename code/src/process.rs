use crate::operations;
use crate::utils::{read_csv, save_csv};
pub fn retranspose(file: &str, output_file: &str) {
    if let Some(data) = read_csv(file, true) {
        let data_transposed = operations::transpose_2d_array(&data);
        match save_csv(&data_transposed.0, &data_transposed.1, ".", output_file) {
            Ok(_) => println!("Saved transpose to {}", output_file),
            Err(err) => println!("{}", err),
        };
    } else {
        println!("Transpose failed!");
    }
}
