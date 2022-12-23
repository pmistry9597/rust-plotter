use super::{orig::{Raw, RawFns}, change_desrip::ChangeDescrip, processor::Processor, notify_hook::NotifyHook};

pub struct Transform<'r, Out, Transformer, Notifier, RawType, RawStore> 
    where 
        RawStore: RawFns<RawType>,
        Transformer: Processor<Out, RawType, RawStore>,
        Notifier: NotifyHook,
{
    out: Out,
    raw: &'r Raw<RawType, RawStore>,
    processor: Transformer,
    notifier: Notifier,
}

impl<'r, Out, Transformer, Notifier, RawType, RawStore> Transform<'r, Out, Transformer, Notifier, RawType, RawStore> 
    where 
        RawStore: RawFns<RawType>,
        Transformer: Processor<Out, RawType, RawStore>,
        Notifier: NotifyHook,
{
    pub fn change(self: &mut Self, change: ChangeDescrip) {
        self.processor.change(self.raw, &mut self.out, &change);
        self.notifier.notify(&change);
    }
    pub fn get_out(self: &Self) -> &Out {
        &self.out
    }
}