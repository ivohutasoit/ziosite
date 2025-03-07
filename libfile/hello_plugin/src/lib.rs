use std::{collections::HashMap, error::Error};

use zoisite_plugin_core::Plugin;

#[derive(Debug)]
pub struct HelloPlugin;

impl HelloPlugin {
    #[unsafe(no_mangle)]
    fn new() -> Box<dyn Plugin> {
        Box::new(Self)
    }
}

impl Plugin for HelloPlugin {
    fn execute(&self, params: &mut HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
        println!("execute with parameters: {:?}", params);
        let mut map  = HashMap::new();
        map.insert("plugin".to_string(), "HelloPlugin".to_string());
        Ok(map)
    }  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut params = HashMap::new();
        params.insert("info".to_string(), "caller".to_string());
        let plugin = HelloPlugin::new();
        let _ = plugin.execute(&mut params);
    }
}
