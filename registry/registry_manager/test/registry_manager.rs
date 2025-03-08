trait TestTrait {
    fn foo(&self) -> u32;
}

#[derive(Default)]
struct MyStruct;

#[zoisite_registry_manager::register(default)]
impl TestTrait for MyStruct {
    fn foo(&self) -> u32 {
        123
    }
}

#[zoisite_registry_manager::registry(TestTrait)]
static TESTTRAIT_REGISTRY: () = ();

enum MyEnum {
    #[allow(unused)]
    MyEnumVariant,
}

#[zoisite_registry_manager::register]
impl TestTrait for MyEnum {
    fn foo(&self) -> u32 {
        456
    }
}

#[test]
fn main() {
    assert_eq!(2, TESTTRAIT_REGISTRY.iter().count());
    assert_eq!(1, TESTTRAIT_REGISTRY.instanciate_all().count());

    let instance = TESTTRAIT_REGISTRY.instanciate_all().next().unwrap();
    assert_eq!(instance.foo(), 123);
}