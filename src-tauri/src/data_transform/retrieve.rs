use super::change_desrip::Accessor;

pub trait Retrieve<T> {
    fn get<'r>(self: &'r Self, accessor: Accessor) -> Box<dyn Iterator<Item = &T> + 'r>;
}