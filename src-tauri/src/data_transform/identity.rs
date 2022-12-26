use std::marker::PhantomData;

use super::{mutate_info::{MutateInfo, Accessor, Mutation}, Retrieve, len::Len};

pub struct Identity<T, C> 
    where C: IdentityContainer<T>
{
    contain: C,
    _t: PhantomData<T>,
}

impl<T,C> Identity<T,C> 
    where C: IdentityContainer<T>
{
    pub fn new(contain: C) -> Self {
        Identity{contain, _t: PhantomData}
    }
    pub fn add(self: &mut Self, entry_iter: impl Iterator<Item = T>) -> MutateInfo {
        let accessor = self.contain.add(entry_iter);
        MutateInfo::Change(vec![Mutation::Add(accessor)])
    }
    pub fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize> + Clone) -> MutateInfo {
        self.contain.remove(index_iter.clone());
        MutateInfo::Change(vec![Mutation::Remove(Accessor::from_indices(index_iter))])
    }
    pub fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)> + Clone) -> MutateInfo {
        self.contain.replace(assoc_iter.clone());
        MutateInfo::Change(vec![Mutation::Replace(Accessor::from_assoc(assoc_iter))])
    }
    pub fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)> + Clone) -> MutateInfo {
        self.contain.insert(assoc_iter.clone());
        MutateInfo::Change(vec![Mutation::Insert(Accessor::from_assoc(assoc_iter))])
    }
    pub fn reset(self: &mut Self) -> MutateInfo {
        MutateInfo::Reset
    }
}

impl<T, C> Retrieve<T> for Identity<T, C> 
    where T: Clone,
        C: IdentityContainer<T> + Retrieve<T>
{
    fn get(self: &Self, accessor: &Accessor) -> Vec<T> {
        self.contain.get(accessor)
    }
}

impl<T, C> Len for Identity<T, C>
    where C: IdentityContainer<T> + Len
{
    fn len(self: &Self) -> usize {
        self.contain.len()
    }
}

pub trait IdentityContainer<T> {
    fn add(self: &mut Self, entry_iter: impl Iterator<Item = T>) -> Accessor;
    fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize>);
    fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)>);
    fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)>);
    fn reset(self: &mut Self);
}


impl<T> IdentityContainer<T> for Vec<T> {
    fn add(self: &mut Self, entry_iter: impl Iterator<Item = T>) -> Accessor {
        let init_size = self.len();
        self.extend(entry_iter);
        Accessor::Range((init_size, self.len()))
    }
    fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize>) {
        index_iter.for_each(|index| {(self as &mut Vec<T>).remove(index);});
    }
    fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)>) {
        assoc_iter.for_each(|(index, entry)| (self as &mut Vec<T>).insert(index, entry));
    }
    fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, T)>) {
        assoc_iter.for_each(|(index, entry)| self[index] = entry);
    }
    fn reset(self: &mut Self) {
        self.clear();
    }
}

impl<T> Retrieve<T> for Vec<T> 
where T: Clone
{
    fn get(self: &Self, accessor: &Accessor) -> Vec<T> {
        match accessor {
            Accessor::Range((begin, end)) => self[*begin..*end].to_vec(),
            Accessor::Indices(indices) => indices.into_iter().map(|index| self[*index].clone()).collect(),
        }
    }
}

impl<T> Len for Vec<T> {
    fn len(self: &Self) -> usize {
        (self as &Vec<T>).len()
    }
}