use super::{Retrieve, mutate_info::MutateInfo};

pub trait Mutator<T, Out>
    where T: Clone
{
    fn mutate<Source: Retrieve<T>>(self: &mut Self, src: &Source, out: &mut Out, change: &MutateInfo) -> MutateInfo;
}