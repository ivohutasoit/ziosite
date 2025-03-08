use std::collections::HashMap;

use clap::Parser;
use zoisite_core::Plugin;
use zoisite_plugin_manager::PluginManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long = "library-path")]
    library_path: String,

    #[arg(short = 'p', long = "plugin")]
    plugin_name: String
}

fn main() {
    let args = Args::parse();
    
    println!("Run {} from {}!", args.plugin_name, args.library_path);
    match PluginManager::new(args.library_path.as_str(), args.plugin_name.as_str()) {
        Ok(plugin) => {
            let mut params = HashMap::new();
            params.insert("caller".to_string(), "index".to_string());
            let result = match plugin.execute(&mut params) {
                Ok(result) => result,
                Err(e) => panic!("could not execute method: {e}")
            };
            println!("{}", format!("{:?}", result))
        },
        Err(e) => println!("{e}")
    }
}
