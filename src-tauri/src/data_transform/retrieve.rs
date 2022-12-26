use futures::lock::MutexGuard;

use super::{mutate_info::Accessor, len::Len};

pub trait Retrieve<T>: Len
    where T: Clone
{
    fn get(self: &Self, accessor: &Accessor) -> Vec<T>;
}

impl<'a, T, S> Retrieve<T> for MutexGuard<'a, S> 
    where T: Clone,
        S: Len + Retrieve<T>,
{
    fn get(self: &Self, accessor: &Accessor) -> Vec<T> {
        (self as &S).get(accessor)
    }
}

impl<'a, S> Len for MutexGuard<'a, S> 
    where S: Len
{
    fn len(self: &Self) -> usize {
        (self as &S).len()
    }
}