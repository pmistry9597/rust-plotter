use super::{change_desrip::Accessor, len::Len};

pub trait Retrieve<T>: Len {
    fn get<'a>(self: &'a Self, accessor: &'a Accessor) -> Box<dyn Iterator<Item = &T> + 'a>;
}