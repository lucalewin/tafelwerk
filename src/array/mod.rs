use std::marker::PhantomData;

pub struct ArrayBase<T, D> {
    items: Vec<T>,
    _dim: PhantomData<D>,
}

pub struct Array {}

impl Array {
    pub fn zeros() -> Self {
        Self {}
    }

    pub fn empty() -> Self {
        Self {}
    }
}
