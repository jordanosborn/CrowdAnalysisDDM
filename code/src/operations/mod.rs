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

fn mean(arr: &[Option<af::Array<crate::RawType>>]) -> Option<af::Array<crate::RawType>> {
    let dims = arr[0].clone()?;
    let dims = dims.dims();
    let size = arr.len();
    let mut array = Vec::with_capacity(size);
    for v in arr {
        array.push((v.clone())?);
    }
    Some(
        array
            .into_par_iter()
            .reduce(move || af::Array::new_empty(dims), |a, f| a + f)
            / (size as crate::RawType),
    )
}

pub fn activity(arr: &[Option<af::Array<crate::RawType>>]) -> Option<f64> {
    let mean_image = mean(arr)?;
    let dims = arr[0].clone()?;
    let dims = dims.dims();
    let size = arr.len();
    let mut array = Vec::with_capacity(size);
    for v in arr {
        array.push((v.clone())?);
    }
    let a = array.into_par_iter().reduce(
        move || af::Array::new_empty(dims),
        |a, f| {
            let m = f - mean_image.clone();
            a + af::mul(&m, &m, true)
        },
    ) / ((size - 1) as crate::RawType);
    Some(af::sum_all(&a).0)
}

pub fn sub_array<T: af::HasAfEnum>(
    arr: &af::Array<T>,
    top_left: (u64, u64),
    bottom_right: (u64, u64),
) -> Option<af::Array<T>> {
    let dims = arr.dims();
    if top_left.0 <= bottom_right.0
        && top_left.1 <= bottom_right.1
        && bottom_right.0 <= dims[0]
        && bottom_right.1 <= dims[1]
    {
        let seq = &[
            af::Seq::new(top_left.0 as u32, bottom_right.0 as u32 - 1, 1),
            af::Seq::new(top_left.1 as u32, bottom_right.1 as u32 - 1, 1),
        ];
        Some(af::index(arr, seq))
    } else {
        None
    }
}

pub fn add_deque(
    a1: Option<VecDeque<af::Array<crate::RawType>>>,
    a2: Option<VecDeque<af::Array<crate::RawType>>>,
) -> Option<VecDeque<af::Array<crate::RawType>>> {
    match (a1, a2) {
        (Some(a1_unwrapped), Some(a2_unwrapped)) => Some(
            a1_unwrapped
                .iter()
                .zip(a2_unwrapped.iter())
                .map(|(x, y)| x + y)
                .collect(),
        ),
        (Some(a), None) | (None, Some(a)) => Some(a),
        _ => None,
    }
}

pub trait As<T> {
    fn from(v: T) -> Self;
}

impl As<usize> for crate::RawType {
    fn from(v: usize) -> Self {
        v as crate::RawType
    }
}

pub fn transpose<T: Clone>(arr: Vec<Vec<T>>) -> Vec<VecDeque<T>> {
    assert!(!arr.is_empty() && !arr[0].is_empty());
    let mut output: Vec<VecDeque<T>> = vec![VecDeque::with_capacity(arr.len()); arr[0].len()];
    for (_, v) in arr.iter().enumerate() {
        for (i, value) in v.iter().enumerate() {
            if let Some(x) = output.get_mut(i) {
                x.push_back(value.clone());
            }
        }
    }
    output
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
    //arr[tau: array] and annuli[radius:(radius, sum, annulus)]
    println!("Started radial averaging!");
    //TODO: this function is a minefield consumes far too many resources
    //crashes often here
    let res = arr
        .to_owned()
        .par_iter()
        .enumerate()
        .map(|(i, a)| {
            let res = annuli
                .to_owned()
                .par_iter()
                .map(|(q, annulus)| {
                    let multiplied = annulus * a;
                    (*q, af::mean_all(&multiplied).0 as crate::RawType)
                })
                .collect();
            println!("Radial averaged tau = {}!", i + 1);
            res
        })
        .collect::<Vec<_>>();

    println!("Radial averaged all time steps!");
    res
}

pub struct Data<T: arrayfire::HasAfEnum> {
    pub time_delta: f64,
    pub data: VecDeque<af::Array<T>>,
    pub capacity: Option<usize>,
}

impl<T: arrayfire::HasAfEnum> IntoIterator for Data<T> {
    type Item = arrayfire::Array<T>;
    type IntoIter = ::std::collections::vec_deque::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
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
    if !arr.is_empty() {
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
    it.into_par_iter()
        .map(|r| {
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
