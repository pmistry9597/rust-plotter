#[derive(Clone, PartialEq)]
pub enum MutateInfo {
    None,
    Reset,
    Change(Vec<Mutation>),
}

impl MutateInfo {
    pub fn new_add(reach: usize, len: usize) -> Self {
        Self::Change(vec![Mutation::Add(Accessor::reverse_range(reach, len))])
    }
    pub fn new_add_single(len: usize) -> Self {
        Self::Change(vec![Mutation::Add(Accessor::single(len - 1))])
    }
}

#[derive(Clone, PartialEq)]
pub enum Mutation {
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
    pub fn to_indices(&self) -> Box<dyn Iterator<Item = usize> + '_> {
        match self {
            Accessor::Range((begin, end)) => Box::new((*begin..*end).into_iter()),
            Accessor::Indices(indices) => Box::new(indices.iter().map(|index| *index))
        }
    }
    pub fn cap(&mut self, max: usize) {
        match self {
            Accessor::Range((begin, end)) => *self = Accessor::Range((*begin, (*end).clamp(0, max))),
            Accessor::Indices(indices) => {
                *indices.last_mut().expect("nothing here??") = (*indices.last().expect("bro help")).clamp(0, max);
                *self = Accessor::Indices(indices.to_vec());
            }
        }
    }
    pub fn extend_left(&mut self, by: usize) {
        match self {
            Accessor::Range((begin, end)) => *self = Accessor::Range(((*begin as i32 - by as i32).max(0).try_into().expect("you failed still loser"), *end)),
            Accessor::Indices(indices) => indices.insert(0, (indices[0] as i32 - by as i32).max(0).try_into().expect("you failed still again loser"))
        }
    }
    pub fn reverse_range(reach: usize, len: usize) -> Self {
        Accessor::Range((len - reach, len))
    }
    pub fn single(index: usize) -> Self {
        Self::Indices(vec![index])
    }
}