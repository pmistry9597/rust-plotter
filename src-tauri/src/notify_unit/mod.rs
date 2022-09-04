use std::{collections::HashMap, sync::Arc, fmt::Debug};
use serde::Serialize;
use tauri::{Window};

#[tauri::command]
pub fn get_unit<URType>(index: i32, name: &str, urmap: tauri::State<URMap<URType>>)
where
    URType: Send + Sync
{

}

// make testing the tauri::window::Window easier duh
pub trait WindowEmit<S: Serialize + Clone + Debug + PartialEq> {
    fn emit(&self, event: &str, payload: S) -> tauri::Result<()>;  
}
impl<S: Serialize + Clone + Debug + PartialEq> WindowEmit<S> for Window {
    fn emit(&self, event: &str, payload: S) -> tauri::Result<()> {
        Window::emit(self, event, payload)
    }
}

pub trait UnitRepo<URType> {
    fn retrieve(self: &Self, index: i32) -> URType;
}
pub type URHook<URType> = Arc<dyn UnitRepo<URType> + Send + Sync>;
pub struct URMap<URType> {
    name_repo: HashMap<&'static str, URHook<URType>>,
    urtype_name: HashMap<URType, &'static str>,
}
impl<URType> URMap<URType> {
    pub fn notify<Window>(self: &Self, index: i32, window: Window)
    where
        URType: Serialize + Clone + Debug + PartialEq,
        Window: WindowEmit<URType>,
    {
    
    }
    pub fn new(map_info: Vec<(URType, URTypeInfo<URType>)>) -> URMap<URType> {
        // TODO
        URMap{ name_repo: HashMap::new(), urtype_name: HashMap::new(), }
    }
}

pub struct URTypeInfo<URType> {
    pub name: &'static str,
    pub repo: URHook<URType>,
}

// system for tauri frontend to receive a unit/chunk of data via indexing
// and notifying frontend when new data is available
// (maybe notify when frontend should poll on its own for frequent updates?)


#[cfg(test)]
mod full_tests {
    use std::{sync::Arc, fmt::Debug};
    use serde::Serialize;

    use crate::notify_unit::{URHook, UnitRepo, URTypeInfo};

    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct Yum;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct Cum;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct Bum;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    enum UnitType {
        Yum(Option<Yum>),
        Cum(Option<Cum>),
        Bum(Option<Bum>),
    }
    impl UnitRepo<UnitType> for Yum {
        fn retrieve(self: &Self, index: i32) -> UnitType {
            // TODO
            UnitType::Yum(Some(Yum))
        }
    }

    // mock window 4 testing lul
    struct MockWindow {
        pub expec_event: &'static str,
        pub expec_payload: UnitType,
    }
    impl WindowEmit<UnitType> for MockWindow {
        fn emit(&self, event: &str, payload: UnitType) -> tauri::Result<()> {
            assert_eq!(event, self.expec_event);
            assert_eq!(payload, self.expec_payload);
            Ok(())
        }
    }

    use super::{URMap, WindowEmit};

    #[test]
    fn notify() {
        let map_info = vec![
            (UnitType::Yum(None), URTypeInfo{name: "yum", repo: Arc::new(Yum)}),
        ];
        let urmap = URMap::<UnitType>::new(map_info);
        let mock_window = MockWindow{ expec_event: "fake_event", expec_payload: UnitType::Yum(None) };

        urmap.notify(0, mock_window);
        // check above called emit and other shit
    }

    #[test]
    fn insert_notify_retrieve() {

    }
}


// figure out how to test without so many freaking bounds on actual code (not tests/mocks)