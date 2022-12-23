use super::{change_desrip::ChangeDescrip, orig::{Raw, RawFns}};

pub trait Processor<Out, RawType, RawStore>
    where RawStore: RawFns<RawType>
{
    fn change(self: &mut Self, raw: &Raw<RawType, RawStore>, out: &mut Out, change: &ChangeDescrip);
}