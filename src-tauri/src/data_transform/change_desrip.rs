#[derive(Clone, PartialEq)]
pub enum ChangeDescrip {
    Reset,
    Change(Vec<Change>),
}

#[derive(Clone, PartialEq)]
pub enum Change {
    Add(Accessor),
    Replace(Accessor),
    Insert(Accessor),
    Remove(Accessor),
}

#[derive(Clone, PartialEq)]
pub enum Accessor {
    Range((usize, usize)),
    Indices(Vec<usize>),
}

impl Accessor {
    pub fn from_indices(index_iter: impl Iterator<Item = usize>) -> Accessor {
        Accessor::Indices(index_iter.collect())
    }
    pub fn from_assoc<_T>(assoc_iter: impl Iterator<Item = (usize, _T)>) -> Accessor {
        Accessor::Indices(assoc_iter.map(|(index, _t)| {index}).collect())
    }
    pub fn to_indices(accessor: Accessor) -> Box<dyn Iterator<Item = usize>> {
        match accessor {
            Accessor::Range((begin, end)) => Box::new((begin..end).into_iter()),
            Accessor::Indices(indices) => Box::new(indices.into_iter(),)
        }
    }
}