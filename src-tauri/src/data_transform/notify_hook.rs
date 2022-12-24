use super::change_desrip::ChangeDescrip;

pub trait NotifyHook {
    fn notify(self: &Self, change: &ChangeDescrip);
}

pub struct EmptyNotifyHook;
impl NotifyHook for EmptyNotifyHook {
    fn notify(self: &Self, _change: &ChangeDescrip) {}
}