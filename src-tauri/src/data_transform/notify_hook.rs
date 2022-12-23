use super::change_desrip::ChangeDescrip;

pub trait NotifyHook {
    fn notify(self: &Self, change: &ChangeDescrip);
}