use std::marker::PhantomData;

use super::{mutate_info::{MutateInfo}, Retrieve, len::Len, mutator::Mutator};

// rationale is to give as much information to whatever transforms the data
// so that most efficient operations and notifications can be executed
pub trait Transform<T, O>: Retrieve<O>
    where T: Clone,
        O: Clone,
{
    fn mutate<Source: Retrieve<T>>(self: &mut Self, src: &Source, change: &MutateInfo) -> MutateInfo;
}

pub struct VecTransform<T, O, M> 
    where M: Mutator<T, Vec<O>>,
        O: Clone,
        T: Clone,
{
    out: Vec<O>,
    mutator: M,
    _t: PhantomData<T>
}

impl<T, O, M> VecTransform<T, O, M>  
    where M: Mutator<T, Vec<O>>,
        T: Clone,
        O: Clone,
{
    pub fn new(out: Vec<O>, mutator: M) -> Self {
        Self{out, mutator, _t: PhantomData}
    }
    pub fn new_empty(mutator: M) -> Self {
        Self::new(vec![], mutator)
    }
}

impl<T, O, M> Transform<T, O> for VecTransform<T, O, M> 
    where M: Mutator<T, Vec<O>>,
        T: Clone,
        O: Clone,
{
    fn mutate<Source: Retrieve<T>>(self: &mut Self, src: &Source, change: &MutateInfo) -> MutateInfo {
        self.mutator.mutate(src, &mut self.out, change)
    }
}

impl<T, O, M> Retrieve<O> for VecTransform<T, O, M> 
    where M: Mutator<T, Vec<O>>,
        T: Clone,
        O: Clone,
{
    fn get(self: &Self, accessor: &super::mutate_info::Accessor) -> Vec<O> {
        accessor.to_indices().map(|index| self.out[index].clone()).collect()
    }
}

impl<T, O, M> Len for VecTransform<T, O, M> 
    where M: Mutator<T, Vec<O>>,
        T: Clone,
        O: Clone,
{
    fn len(self: &Self) -> usize {
        self.out.len()
    }
}