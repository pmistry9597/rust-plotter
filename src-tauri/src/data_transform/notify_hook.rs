use super::change_desrip::ChangeDescrip;

pub trait NotifyHook {
    fn notify(self: &mut Self, change: &ChangeDescrip);
}

pub struct EmptyNotifyHook;
impl NotifyHook for EmptyNotifyHook {
    fn notify(self: &mut Self, _change: &ChangeDescrip) {}
}