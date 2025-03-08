use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_str, Ident, ItemImpl, ItemStatic};

struct RegisterItem {
    item: ItemImpl,
}

impl Parse for RegisterItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: ItemImpl::parse(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn register(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let constructor_fn = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr as RegisterAttribute))
    };

    let has_constructor = constructor_fn.is_some();
    let has_constructor = quote! { #has_constructor };

    let constructor_fn_call_str = if let Some(cfn) = constructor_fn {
        let ident = cfn.constructor_fn_ident;
        quote! {
            Some(Box::new(Self::#ident()))
        }
    } else {
        quote! {
            None
        }
    };

    let item_clone = item.clone();

    let parsed_item = parse_macro_input!(item as RegisterItem);
    let item_impl = parsed_item.item;

    let (trait_not, trait_path, _) = item_impl
        .trait_
        .expect("Can only register an implementation of a trait, 'impl <Trait> for <Type>'.");
    assert!(
        trait_not.is_none(),
        "Cannot register inverted impl trait: 'impl !Trait for Type'."
    );

    let trait_ident = trait_path
        .require_ident()
        .expect("Expected trait in impl block to have an identifier.");
    let trait_name = format!("{trait_ident}");

    let type_path = get_self_type_path(&item_impl.self_ty);
    let type_ident = type_path
        .require_ident()
        .expect("Expected type in impl block to have an identifier.");
    let type_name = format!("{type_ident}");

    let register_static_ident =
        parse_str::<Ident>(format!("{}_{}__Register", type_ident, trait_ident).as_ref())
            .expect("Unable to create identifier");
    let register_static_fn_ident = parse_str::<Ident>(
        format!("{}_{}__RegisterFn", type_ident, trait_ident).as_ref(),
    )
    .expect("Unable to create identifier");

    let mut result: proc_macro::TokenStream = quote! {
        impl zoisite_registry_manager::RegistryImplementation<Box<dyn #trait_path>> for #type_path {
            const INITIATE: fn() -> Option<Box<dyn #trait_path>> = || { #constructor_fn_call_str };
            const HAS_CONSTRUCTOR: bool = #has_constructor;
            const NAME: &'static str = #type_name;
            const PATH: &'static str = stringify!(#type_path);
            const FILE: &'static str = core::file!() ;
            const MODULE_PATH: &'static str = core::module_path!();
            const TRAIT_NAME: &'static str = #trait_name;
        }

        #[used]
        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "freebsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "netbsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "openbsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "dragonfly", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "illumos", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "haiku", link_section = ".init_array.10000")]
        #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(windows, link_section = ".CRT$XCT")]
        static #register_static_ident: extern fn() = {
            extern fn #register_static_fn_ident() {
                zoisite_registry_manager::__register_implementation::<Box<dyn #trait_path>, #type_path>();
            }
            #register_static_fn_ident
        };
    }.into();

    result.extend(item_clone.clone());

    result
}

#[derive(Debug)]
struct RegistryAttribute {
    trait_ident: Ident,
}

impl Parse for RegistryAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_ident: Ident::parse(input)?,
        })
    }
}

struct RegistryItem {
    item: ItemStatic,
}

impl Parse for RegistryItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: ItemStatic::parse(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn registry(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let registry_attr = syn::parse_macro_input!(attr as RegistryAttribute);
    let registry_item = syn::parse_macro_input!(item as RegistryItem);

    let trait_ident = registry_attr.trait_ident;
    let item = registry_item.item;

    let trait_name = format!("{trait_ident}");
    let item_ident = item.ident;
    let storage_ident = parse_str::<Ident>(format!("{}__STORAGE", item_ident).as_ref())
        .expect("Unable to create identifier");
    let wrapper_struct_ident =
        parse_str::<Ident>(format!("{}__Registry", item_ident).as_ref())
            .expect("Unable to create identifier");
    let build_static_ident =
        parse_str::<Ident>(format!("{}__Build", item_ident).as_ref())
            .expect("Unable to create identifier");
    let build_static_fn_ident =
        parse_str::<Ident>(format!("{}__BuildFn", item_ident).as_ref())
            .expect("Unable to create identifier");

    quote! {
        static mut #storage_ident: Option<zoisite_registry_manager::RegistryStorage<Box<dyn #trait_ident>>> = None;

        static #item_ident: #wrapper_struct_ident = #wrapper_struct_ident {};

        struct #wrapper_struct_ident;

        impl ::core::ops::Deref for #wrapper_struct_ident {
            type Target = zoisite_registry_manager::RegistryStorage<Box<dyn #trait_ident>>;
            fn deref(&self) -> &'static zoisite_registry_manager::RegistryStorage<Box<dyn #trait_ident>> {
                unsafe {
                    #storage_ident.as_ref().unwrap()
                }
            }
        }

        #[used]
        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "freebsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "netbsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "openbsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "dragonfly", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "illumos", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "haiku", link_section = ".init_array.20000")]
        #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(windows, link_section = ".CRT$XCU")]
        static #build_static_ident: extern fn() = {
            extern fn #build_static_fn_ident() {
                let storage = zoisite_registry_manager::RegistryStorage::<Box<dyn #trait_ident>>::__new(#trait_name);

                unsafe {
                    #storage_ident = Some(storage)
                }
            }
            #build_static_fn_ident
        };
    }.into()
}

#[derive(Debug)]
struct RegisterAttribute {
    constructor_fn_ident: Ident,
}

impl Parse for RegisterAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            constructor_fn_ident: Ident::parse(input)?,
        })
    }
}

fn get_self_type_path(self_ty: &syn::Type) -> &syn::Path {
    if let syn::Type::Path(type_path) = self_ty {
        return &type_path.path;
    }

    let error_type = match self_ty {
        syn::Type::Array(_) => "n array",
        syn::Type::BareFn(_) => " function",
        syn::Type::Group(_) => " group",
        syn::Type::ImplTrait(_) => " trait impl",
        syn::Type::Infer(_) => "n inferred type (_)",
        syn::Type::Macro(_) => " macro",
        syn::Type::Never(_) => " never type",
        syn::Type::Paren(_) => " parenthesis",
        syn::Type::Ptr(_) => " pointer",
        syn::Type::Reference(_) => " reference",
        syn::Type::Slice(_) => " slice",
        syn::Type::TraitObject(_) => " trait object",
        syn::Type::Tuple(_) => " tuple",
        syn::Type::Verbatim(_) => "n unknown syntax",
        _ => unreachable!(),
    };

    panic!(
        "Cannot register implementation on a{}, expected a struct, enum, union or type alias.",
        error_type
    );
}