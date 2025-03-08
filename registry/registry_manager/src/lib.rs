pub use zoisite_registry_manager_macros::{register, registry};

static __REGISTRY: std::sync::Mutex<Vec<RegistryImplementationWrapper<Box<u32>>>> =
    std::sync::Mutex::new(vec![]);

pub trait RegistryImplementation<Trait> {
    const INITIATE: fn() -> Option<Trait>;
    const HAS_CONSTRUCTOR: bool;
    const NAME: &'static str;
    const PATH: &'static str;
    const FILE: &'static str;
    const MODULE_PATH: &'static str;
    const TRAIT_NAME: &'static str;
}

pub fn __register_implementation<Trait, Type: RegistryImplementation<Trait>>() {
    let wrapper = RegistryImplementationWrapper::<Trait> {
        initiate: Type::INITIATE,
        has_constructor: Type::HAS_CONSTRUCTOR,
        name: Type::NAME,
        path: Type::PATH,
        file: Type::FILE,
        module_path: Type::MODULE_PATH,
        trait_name: Type::TRAIT_NAME,
    };
    let wrapper: RegistryImplementationWrapper<Box<u32>> = unsafe { core::mem::transmute(wrapper) };

    let mut registry_ref = __REGISTRY
        .lock()
        .expect("Registry internal mutex poisoned");
    registry_ref.push(wrapper);
}

pub struct RegistryStorage<Trait> {
    impls: Vec<RegistryImplementationWrapper<Trait>>,
}

impl<Trait> RegistryStorage<Trait> {
    #[doc(hidden)]
    pub fn __new(trait_: &'static str) -> Self {
        let registry_ref = __REGISTRY
            .lock()
            .expect("Registry internal mutex poisoned");

        let impls = registry_ref
            .iter()
            .filter(|item| item.trait_name == trait_)
            .cloned()
            .map(|item| {
                let item: RegistryImplementationWrapper<Trait> = unsafe { core::mem::transmute(item) };
                item
            })
            .collect();

        Self { impls }
    }

    pub fn iter(&self) -> core::slice::Iter<RegistryImplementationWrapper<Trait>> {
        self.impls.iter()
    }

    pub fn initiate_all(&self) -> impl Iterator<Item = Trait> + '_ {
        self.impls.iter().filter_map(|item| item.initiate())
    }
}

#[derive(Clone)]
pub struct RegistryImplementationWrapper<Trait> {
    initiate: fn() -> Option<Trait>,
    has_constructor: bool,
    name: &'static str,
    path: &'static str,
    file: &'static str,
    module_path: &'static str,
    trait_name: &'static str,
}

impl<Trait> RegistryImplementationWrapper<Trait> {
    pub fn initiate(&self) -> Option<Trait> {
        (self.initiate)()
    }

    pub fn has_constructor(&self) -> bool {
        self.has_constructor
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    pub fn trait_name(&self) -> &'static str {
        self.trait_name
    }
}

impl<Trait> core::fmt::Debug for RegistryImplementationWrapper<Trait> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("RegistryImplementation")
            .field("Name", &self.name)
            .field("Path", &self.path)
            .field("Trait Name", &self.trait_name)
            .field("Has Constructor", &self.has_constructor)
            .field("Module Path", &self.module_path)
            .field("File", &self.file)
            .finish()
    }
}
