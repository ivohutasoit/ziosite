# Zoisite Project

This project is a collection of plugins based on Rust implementation. It provides a framework for building and managing plugins in Rust applications.

## Getting Started

To get started with the project, follow these steps:

1. Clone the repository: https://github.com/ivohutasoit/zoisite-project.git
2. Navigate to the root directory of the project.
3. Run `cargo build` to compile the project.
4. You can now use the compiled plugins in your Rust application.
5. Run this commend below to execute the main program.
   ```sh
   cargo run --bin zoisite_plugin_runner -- -l ${PWD}/target/debug -p zoisite_hello_plugin # for plugin based runner

   cargo run --bin zoisite_registry_runner # for registry based runner
   ```
