use std::{collections::HashMap, error::Error};

use libloading::{Library, Symbol};
use zoisite_core::Plugin;


pub type PluginCreator = unsafe fn() -> Box<dyn Plugin>;

#[derive(Debug)]
pub struct PluginManager {
    plugin: Box<dyn Plugin>,
    _lib: Library
}

impl PluginManager {
    pub fn new(path: &str, name: &str) -> Result<Self, Box<dyn Error>> {
        #[cfg(target_os = "linux")]
        let os_lib_name = format!("lib{name}.so");
        #[cfg(target_os = "macos")]
        let os_lib_name = format!("lib{name}.dylib");
        #[cfg(target_os = "windows")]
        let os_lib_name = format!("{name}.dll");

        let lib_path = format!("{path}/{os_lib_name}");
        let _lib = unsafe { Library::new(lib_path)? };
        let creator: Symbol<PluginCreator> = unsafe { _lib.get(b"new")? };

        let plugin = unsafe { creator() };
        Ok(Self { _lib, plugin})
    }
}

impl Plugin for PluginManager {
    fn execute(&self, params: &mut HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
        self.plugin.execute(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct TestPlugin;

    impl TestPlugin {
        #[unsafe(no_mangle)]
        fn new() -> Box<dyn Plugin> {
            Box::new(Self)
        }
    }

    impl Plugin for TestPlugin {
        fn execute(&self, params: &mut HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
            println!("execute with parameters: {:?}", params);
            let mut map  = HashMap::new();
            map.insert("plugin".to_string(), "Test".to_string());
            Ok(map)
        }  
    }

    #[test]
    fn it_works() {
        let mut params = HashMap::new();
        params.insert("info".to_string(), "caller".to_string());
        let plugin = TestPlugin::new();
        let _ = plugin.execute(&mut params);
    }
}
