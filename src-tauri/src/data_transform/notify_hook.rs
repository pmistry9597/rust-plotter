use super::mutate_info::MutateInfo;

pub trait NotifyHook {
    fn notify(self: &mut Self, change: &MutateInfo);
}