pub enum ChangeDescrip {
    Reset,
    Change(Vec<Change>),
}

pub enum Change {
    Add(Accessor),
    Replace(Accessor),
    Insert(Accessor),
    Remove(Accessor),
}

pub enum Accessor {
    Range((usize, usize)),
    Indices(Vec<usize>),
}

pub fn indices_accessor(index_iter: impl Iterator<Item = usize>) -> Accessor {
    Accessor::Indices(index_iter.collect())
}
pub fn assoc_accessor<_T>(assoc_iter: impl Iterator<Item = (usize, _T)>) -> Accessor {
    Accessor::Indices(assoc_iter.map(|(index, _t)| {index}).collect())
}