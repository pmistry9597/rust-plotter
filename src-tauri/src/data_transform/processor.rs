use super::{change_desrip::ChangeDescrip, store::{StoreFns}, Retrieve};

pub trait Processor<Out, RawType, RawStore>
    where RawStore: StoreFns<RawType>
{
    fn change<StoreType: Retrieve<RawType>>(self: &mut Self, raw: &StoreType, out: &mut Out, change: &ChangeDescrip) -> ChangeDescrip;
}