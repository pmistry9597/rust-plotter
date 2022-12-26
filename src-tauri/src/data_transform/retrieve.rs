use super::{mutate_info::Accessor, len::Len};

pub trait Retrieve<T>: Len
    where T: Clone
{
    fn get(self: &Self, accessor: &Accessor) -> Vec<T>;
}

//Box<dyn Iterator<Item = T>>