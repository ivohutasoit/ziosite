use std::{collections::HashMap, error::Error, fmt::Debug};

use zoisite_core::Plugin;

#[derive(Debug, Default)]
struct HelloRegistryPlugin;

#[zoisite_registry_manager::register(default)]
impl Plugin for HelloRegistryPlugin {
    fn execute(&self, params: &mut HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
        println!("execute with parameters: {:?}", params);
        let mut map  = HashMap::new();
        map.insert("plugin".to_string(), "HelloRegistryPlugin".to_string());
        Ok(map)
    }  
}

#[zoisite_registry_manager::registry(Plugin)]
static PLUGIN_REGISTRY: () = ();

fn main() {
    for plugin in PLUGIN_REGISTRY.iter() {
        println!("{plugin:#?}");
        let _ = plugin.name();
        let _ = plugin.path();
        let _ = plugin.file();
        let _ = plugin.trait_name();
        let _ = plugin.module_path();

        if let Some(instance) = plugin.initiate() {
            println!("Instance: {instance:#?}");
        }
    }

    let instances: Vec<Box<dyn Plugin>> = PLUGIN_REGISTRY.initiate_all().collect();

    for instance in instances.iter() {
        let mut params = HashMap::new();
        params.insert("caller".to_string(), "index".to_string());
        let result = match instance.execute(&mut params) {
            Ok(result) => result,
            Err(e) => panic!("could not execute method: {e}")
        };
        println!("{}", format!("{:?}", result))
    }
}
