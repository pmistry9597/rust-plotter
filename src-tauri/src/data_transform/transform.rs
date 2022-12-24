use std::marker::PhantomData;

use super::{orig::{Raw, RawFns}, change_desrip::ChangeDescrip, processor::Processor, notify_hook::NotifyHook};

// rationale is to give as much information to whatever transforms the data
// so that most efficient operations and notifications can be executed
pub struct Transform<Out, Transformer, Notifier, RawType, RawStore> 
    where 
        RawStore: RawFns<RawType>,
        Transformer: Processor<Out, RawType, RawStore>,
        Notifier: NotifyHook,
{
    out: Out,
    // raw: &'r Raw<RawType, RawStore>,
    processor: Transformer,
    notifier: Notifier,
    _raw_phantom: PhantomData<(RawType, RawStore)>,
}

impl<'r, Out, Transformer, Notifier, RawType, RawStore> Transform<Out, Transformer, Notifier, RawType, RawStore> 
    where 
        RawStore: RawFns<RawType>,
        Transformer: Processor<Out, RawType, RawStore>,
        Notifier: NotifyHook,
{
    pub fn new(out: Out, processor: Transformer, notifier: Notifier) -> Self {
        Transform{out, processor, notifier, _raw_phantom: PhantomData}
    }
    pub fn change(self: &mut Self, raw: &'r Raw<RawType, RawStore>, change: &ChangeDescrip) {
        self.processor.change(raw, &mut self.out, change);
        self.notifier.notify(&change);
    }
    pub fn get_out(self: &Self) -> &Out {
        &self.out
    }
}