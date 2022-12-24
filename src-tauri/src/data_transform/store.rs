use std::marker::PhantomData;

use super::{change_desrip::{ChangeDescrip, Change, Accessor}, len::Len, retrieve::Retrieve};

pub struct Store<Type, RawStore> 
    where RawStore: StoreFns<Type>
{
    raw_store: RawStore,
    _t: PhantomData<Type>,
}

impl<Type, RawStore> Store<Type, RawStore> 
    where RawStore: StoreFns<Type>
{
    pub fn new(raw_store: RawStore) -> Self {
        Store{raw_store, _t: PhantomData}
    }
    pub fn add(self: &mut Self, entry_iter: impl Iterator<Item = Type>) -> ChangeDescrip {
        let accessor = self.raw_store.add(entry_iter);
        ChangeDescrip::Change(vec![Change::Add(accessor)])
    }
    pub fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize> + Clone) -> ChangeDescrip {
        self.raw_store.remove(index_iter.clone());
        ChangeDescrip::Change(vec![Change::Remove(Accessor::from_indices(index_iter))])
    }
    pub fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)> + Clone) -> ChangeDescrip {
        self.raw_store.replace(assoc_iter.clone());
        ChangeDescrip::Change(vec![Change::Replace(Accessor::from_assoc(assoc_iter))])
    }
    pub fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)> + Clone) -> ChangeDescrip {
        self.raw_store.insert(assoc_iter.clone());
        ChangeDescrip::Change(vec![Change::Insert(Accessor::from_assoc(assoc_iter))])
    }
    pub fn reset(self: &mut Self) -> ChangeDescrip {
        ChangeDescrip::Reset
    }
}

impl<Type, RawStore> Len for Store<Type, RawStore> 
    where RawStore: StoreFns<Type>
{
    fn len(self: &Self) -> usize {
        self.raw_store.len()
    }
}

impl<T, RawStore> Retrieve<T> for Store<T, RawStore>
    where RawStore: StoreFns<T>
{
    fn get<'a>(self: &'a Self, accessor: &'a Accessor) -> Box<dyn Iterator<Item = &T> + 'a> {
        self.raw_store.get(accessor)
    }
}

pub trait StoreFns<Type>: Len + Retrieve<Type> {
    fn add(self: &mut Self, entry_iter: impl Iterator<Item = Type>) -> Accessor; // assume it returns indices that were added
    fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize>); // assume order is way to remove (check order of the iterator you pass in)
    fn replace(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)>);
    fn insert(self: &mut Self, assoc_iter: impl Iterator<Item = (usize, Type)>); // assume order is way to remove (check order of the iterator you pass in)
    fn reset(self: &mut Self);
}

impl<T> StoreFns<T> for &Vec<T> {
    fn add(self: &mut Self, entry_iter: impl Iterator<Item = T>) -> Accessor {
        let init_size = self.len();
        self.extend(entry_iter);
        Accessor::Range((init_size, self.len()))
    }
    fn remove(self: &mut Self, index_iter: impl Iterator<Item = usize>) {
        index_iter.for_each(|index| {self.remove(index);});
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
impl<T> Len for &Vec<T> {
    fn len(self: &Self) -> usize {
        self.len()
    }
}

impl<T> Retrieve<T> for &Vec<T> {
    fn get<'a>(self: &'a Self, accessor: &'a Accessor) -> Box<dyn Iterator<Item = &T> + 'a> {
        match accessor {
            Accessor::Range((begin, end)) => Box::new(self[*begin..*end].iter()),
            Accessor::Indices(indices) => Box::new(indices.into_iter().map(|index| &self[*index])),
        }
    }
}