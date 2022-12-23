pub trait Retrieve<T> {
    fn get<It: Iterator<Item = T>>(self: &Self, index_iter: impl Iterator<Item = usize>) -> It;
}