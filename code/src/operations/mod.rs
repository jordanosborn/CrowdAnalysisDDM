use crate::RawType;
use arrayfire as af;
use rayon::prelude::*;
use std::collections::VecDeque;

pub fn difference(
    arr1: &arrayfire::Array<crate::RawFtType>,
    arr2: &arrayfire::Array<crate::RawFtType>,
) -> arrayfire::Array<crate::RawType> {
    let abs = arrayfire::abs(&(arr1 - arr2));
    arrayfire::mul(&abs, &abs, true)
}

pub trait As<T> {
    fn from(v: T) -> Self;
}

impl As<usize> for crate::RawType {
    fn from(v: usize) -> Self {
        v as crate::RawType
    }
}

pub fn transpose_2d_array<T: Clone + As<usize>>(arr: &[Vec<(T, T)>]) -> (Vec<T>, Vec<Vec<(T, T)>>) {
    assert!(!arr.is_empty() && !arr[0].is_empty());
    let mut output: Vec<Vec<(T, T)>> = vec![Vec::with_capacity(arr.len()); arr[0].len()];
    let index = arr[0].iter().map(|(q, _)| q.clone()).collect();
    for (j, v) in arr.iter().enumerate() {
        for (i, value) in v.iter().enumerate() {
            if let Some(x) = output.get_mut(i) {
                let replaced = (T::from(j + 1), value.clone().1);
                x.push(replaced);
            }
        }
    }
    (index, output)
}

pub fn radial_average(
    arr: &[arrayfire::Array<RawType>],
    annuli: &[(RawType, arrayfire::Array<RawType>)],
) -> Vec<Vec<(RawType, RawType)>> {
    //TODO: speed this up this is very slow
    let mut vector = Vec::with_capacity(arr.len());
    println!("Started radial averaging!");
    arr.iter().enumerate().for_each(|(i, a)| {
        let average = annuli
            .par_iter()
            .map(|(q, annulus)| {
                (
                    *q,
                    ((arrayfire::sum_all(&(annulus * a)).0) / (arrayfire::sum_all(annulus).0))
                        as crate::RawType,
                )
            })
            .collect::<Vec<(RawType, RawType)>>();
        vector.push(average);
        println!("Radial averaged tau = {}!", i + 1);
    });
    println!("Radial averaged all time steps!");
    vector
}

pub struct Data<T: arrayfire::HasAfEnum> {
    pub time_delta: f64,
    pub data: VecDeque<af::Array<T>>,
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
    pub fn push(&mut self, array: af::Array<T>) {
        if let Some(capacity) = self.capacity {
            if self.data.len() == capacity {
                let v = self.data.pop_front();
                if let Some(dat) = v {
                    drop(dat);
                }
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
                |acc: af::Array<crate::RawType>, x| acc + x,
            ) / arr.len() as crate::RawType,
        )
    } else {
        None
    }
}

//Algorithm tested in python!
fn create_annulus(dimension: u64, radius: u64, thickness: u64) -> arrayfire::Array<crate::RawType> {
    let radius2 = radius * radius;
    let radius_plus_dr2 = (radius + thickness) * (radius + thickness);
    let annulus: Vec<crate::RawType> = (0..(dimension * dimension))
        .into_par_iter()
        .map(|i| {
            let x = i % dimension;
            let y = i / dimension;
            let r2 = (x - dimension / 2) * (x - dimension / 2)
                + (y - dimension / 2) * (y - dimension / 2);
            if radius2 <= r2 && r2 <= radius_plus_dr2 {
                1.0
            } else {
                0.0
            }
        })
        .collect();
    af::Array::new(
        annulus.as_slice(),
        arrayfire::Dim4::new(&[dimension, dimension, 1, 1]),
    )
}

pub fn generate_annuli(
    dimension: u64,
    spacing: u64,
) -> Vec<(RawType, arrayfire::Array<crate::RawType>)> {
    let dimension = dimension;
    let max = (dimension / 2) as usize;
    let it = (1..max).step_by(spacing as usize).collect::<Vec<usize>>();
    it.par_iter()
        .map(|&r| {
            (
                (2 * r + spacing as usize) as RawType / 2.0 as RawType,
                create_annulus(dimension, r as u64, spacing),
            )
        })
        .collect::<Vec<(RawType, arrayfire::Array<crate::RawType>)>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wait;

    #[test]
    fn test_annuli() {
        use crate::utils::save_images;
        crate::set_backend();
        let annuli = generate_annuli(500, 10);
        let a = annuli
            .iter()
            .map(|(_, val)| val.clone())
            .collect::<Vec<af::Array<crate::RawType>>>();
        save_images(&a, String::from("presentation_video"));
        wait!();
    }
}
