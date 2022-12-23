use std::marker::PhantomData;

use super::{change_desrip::{ChangeDescrip, Change, Accessor, assoc_accessor}, len::Len, retrieve::Retrieve};

pub struct Raw<Type, RawStore> 
    where RawStore: RawFns<Type>
{
    raw_store: RawStore,
    _t: PhantomData<Type>,
}

impl<Type, RawStore> Raw<Type, RawStore> 
    where RawStore: RawFns<Type>
{
    pub fn new(raw_store: RawStore) -> Raw<Type, RawStore> {
        Raw{raw_store, _t: PhantomData}
    }
    pub fn add(self: &mut Self, entry_iter: impl Iterator<Item = Type>) -> ChangeDescrip {
        let accessor = self.raw_store.add(entry_iter);
        ChangeDescrip::Change(vec![Change::Add(accessor)])
    }
    pub fn remove(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)> + Clone) -> ChangeDescrip {
        self.raw_store.remove(assoc_iter.clone());
        ChangeDescrip::Change(vec![Change::Remove(assoc_accessor(assoc_iter))])
    }
    pub fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)> + Clone) -> ChangeDescrip {
        self.raw_store.replace(assoc_iter.clone());
        ChangeDescrip::Change(vec![Change::Replace(assoc_accessor(assoc_iter))])
    }
    pub fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)> + Clone) -> ChangeDescrip {
        self.raw_store.insert(assoc_iter.clone());
        ChangeDescrip::Change(vec![Change::Insert(assoc_accessor(assoc_iter))])
    }
    pub fn reset(self: &mut Self) -> ChangeDescrip {
        ChangeDescrip::Reset
    }
}

impl<Type, RawStore> Len for Raw<Type, RawStore> 
    where RawStore: RawFns<Type>
{
    fn len(self: &Self) -> usize {
        self.raw_store.len()
    }
}

impl<Type, RawStore> Retrieve<Type> for Raw<Type, RawStore>
    where RawStore: RawFns<Type>
{
    fn get<It: Iterator<Item = Type>>(self: &Self, index_iter: impl Iterator<Item = usize>) -> It {
        self.raw_store.get(index_iter)
    }
}

pub trait RawFns<Type>: Len + Retrieve<Type> {
    fn add(self: &mut Self, entry_iter: impl Iterator<Item = Type>) -> Accessor; // returns indices that were added
    fn remove(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)>);
    fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)>);
    fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)>);
    fn reset(self: &mut Self);
}