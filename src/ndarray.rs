use std::ops::{Index, Mul, Add};

#[derive(Debug)]
pub struct NdArray {
    items: Vec<f64>,
}

impl NdArray {
    pub fn from_array(arr: &[f64]) -> Self {
        Self {
            items: arr.to_vec(),
        }
    }
}

impl Index<usize> for NdArray {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl Mul<NdArray> for f64 {
    type Output = NdArray;

    fn mul(self, mut rhs: NdArray) -> Self::Output {
        for i in 0..rhs.items.len() {
            rhs.items[i] *= self;
        }
        rhs
    }
}

// impl Add for NdArray {
//     type Output = NdArray;

//     fn add(self, rhs: NdArray) -> Self::Output {
//         todo!()
//     }
// }

// impl Add<&NdArray> for NdArray {
//     type Output = NdArray;

//     fn add(self, rhs: &NdArray) -> Self::Output {
//         todo!()
//     }
// }

impl Add<NdArray> for &NdArray {
    type Output = NdArray;

    fn add(self, rhs: NdArray) -> Self::Output {
        if self.items.len() != rhs.items.len() {
            panic!("wrong length") // FIXME
        }

        let items = self.items.iter()
            .zip(rhs.items)
            .map(|(a, b)| a + b)
            .collect();
        
        NdArray { items }
    }
}
