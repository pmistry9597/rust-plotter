use std::mem;

pub trait SingleConsume<T> {
    fn try_consume(self: &mut Self) -> Option<T>;
}

pub struct SingleConsumable<T> (Option<T>);
impl<T> SingleConsume<T> for SingleConsumable<T> {
    fn try_consume(self: &mut Self) -> Option<T> {
        let consum = &mut self.0;
        match consum {
            None => None,
            Some(_) => mem::replace(consum, None)
        }
    }
}

impl<T> SingleConsumable<T> {
    pub fn new(val: T) -> SingleConsumable<T> {
        SingleConsumable(Some(val))
    }
}