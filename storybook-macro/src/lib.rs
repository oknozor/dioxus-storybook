use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FnArg, Ident, ItemFn, ItemStruct, Pat, Type};

/// Marks a component for inclusion in the storybook.
///
/// The component's Props struct must implement the `Stories` trait
/// to provide story configurations for the storybook UI.
///
/// # Example
/// ```ignore
/// #[storybook(tag = "Thumbnails")]
/// #[component]
/// pub fn RoundedThumbnail(
///     size: u32,
///     name: String,
///     src: String,
/// ) -> Element {
///     // ...
/// }
///
/// #[cfg(feature = "storybook")]
/// impl storybook::Stories for RoundedThumbnailProps {
///     fn stories() -> Vec<storybook::Story<Self>> {
///         vec![
///             storybook::Story::new("Default", Self {
///                 size: 64,
///                 name: "Example".to_string(),
///                 src: "https://picsum.photos/200".to_string(),
///             }),
///             storybook::Story::with_description(
///                 "Large",
///                 "A larger thumbnail variant",
///                 Self {
///                     size: 128,
///                     name: "Large Example".to_string(),
///                     src: "https://picsum.photos/400".to_string(),
///                 }
///             ),
///         ]
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn storybook(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as StorybookArgs);

    // Try to parse as a function first, then as a struct
    let item_clone = item.clone();
    if let Ok(input) = syn::parse::<ItemFn>(item_clone) {
        storybook_for_function(input, attr_args)
    } else if let Ok(input) = syn::parse::<ItemStruct>(item) {
        storybook_for_struct(input, attr_args)
    } else {
        TokenStream::from(
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "storybook attribute can only be applied to functions or structs",
            )
                .to_compile_error(),
        )
    }
}

/// Check if a type is a signal wrapper that can be edited via its inner type
fn is_editable_signal_type(ty: &Type) -> bool {
    let ty_str = quote!(#ty).to_string().replace(" ", "");
    ty_str.starts_with("Signal<")
        || ty_str.starts_with("ReadSignal<")
        || ty_str.starts_with("WriteSignal<")
}

/// Extract the inner type string from a Signal wrapper type
/// e.g., "Signal < bool >" -> Some("bool"), "Signal < String >" -> Some("String")
fn extract_signal_inner_type_str(ty: &Type) -> Option<String> {
    let ty_str = quote!(#ty).to_string().replace(" ", "");

    for prefix in &["Signal<", "ReadSignal<", "WriteSignal<"] {
        if ty_str.starts_with(prefix) {
            // Extract everything between < and the final >
            let inner = &ty_str[prefix.len()..ty_str.len() - 1];
            return Some(inner.to_string());
        }
    }
    None
}

/// Check if a type is non-serializable (EventHandler, Callback, Element, etc.)
/// Note: Signal, ReadSignal, WriteSignal are now editable via their inner type
fn is_non_serializable_type(ty: &Type) -> bool {
    let ty_str = quote!(#ty).to_string().replace(" ", "");
    ty_str.starts_with("EventHandler")
        || ty_str.starts_with("Callback")
        || ty_str == "Element"
        || ty_str.starts_with("Option<Element>")
        // ReadOnlySignal remains non-editable (read-only by design)
        || ty_str.starts_with("ReadOnlySignal<")
        || ty_str.starts_with("Option<Signal<")
        || ty_str.starts_with("Option<ReadSignal<")
        || ty_str.starts_with("Option<WriteSignal<")
        || ty_str.starts_with("Option<ReadOnlySignal<")
        || ty_str.starts_with("Option<EventHandler")
        || ty_str.starts_with("Option<Callback")
        || ty_str.starts_with("Vec<Attribute>")
}

/// Get a human-readable type name for display
fn get_type_display_name(ty: &Type) -> String {
    let ty_str = quote!(#ty).to_string();
    // Simplify common types for display
    if ty_str.starts_with("EventHandler") {
        "EventHandler".to_string()
    } else if ty_str.starts_with("Callback") {
        "Callback".to_string()
    } else if ty_str == "Element" {
        "Element".to_string()
    } else if let Some(inner) = extract_signal_inner_type_str(ty) {
        // For Signal<T>, show the inner type
        inner
    } else if ty_str.starts_with("Vec < Attribute >") {
        "Attributes".to_string()
    } else {
        ty_str
    }
}

fn storybook_for_struct(input: ItemStruct, attr_args: StorybookArgs) -> TokenStream {
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();

    // The struct name should end with "Props", and the component name is without "Props"
    let component_name_str = struct_name_str
        .strip_suffix("Props")
        .unwrap_or(&struct_name_str);
    let component_name = format_ident!("{}", component_name_str);
    let tag = &attr_args.tag;

    let story_props_name = format_ident!("{}StoryProps", component_name_str);
    let render_fn_name =
        format_ident!("__storybook_render_with_props_{}", component_name_str.to_lowercase());
    let get_stories_fn_name =
        format_ident!("__storybook_get_stories_{}", component_name_str.to_lowercase());
    let get_prop_fields_fn_name =
        format_ident!("__storybook_get_prop_fields_{}", component_name_str.to_lowercase());
    let get_prop_schema_fn_name =
        format_ident!("__storybook_get_prop_schema_{}", component_name_str.to_lowercase());

    // Extract fields from the struct
    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return TokenStream::from(
                syn::Error::new_spanned(&input, "storybook only supports structs with named fields")
                    .to_compile_error(),
            );
        }
    };

    // Generate StoryProps fields - map non-serializable types to ()
    // For Signal types, use the inner type instead
    let story_props_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            if is_non_serializable_type(field_ty) {
                quote! { pub #field_name: () }
            } else if let Some(inner_ty_str) = extract_signal_inner_type_str(field_ty) {
                // For Signal<T>, use T as the field type
                let inner_ty: Type = syn::parse_str(&inner_ty_str).expect("Failed to parse inner type");
                quote! { pub #field_name: #inner_ty }
            } else {
                quote! { pub #field_name: #field_ty }
            }
        })
        .collect();

    // Generate field info for the UI
    let field_infos: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_ty = &field.ty;
            let editable = !is_non_serializable_type(field_ty);
            let type_name = get_type_display_name(field_ty);
            quote! {
                storybook::PropFieldInfo {
                    name: #field_name,
                    editable: #editable,
                    type_name: #type_name,
                }
            }
        })
        .collect();

    // Generate conversion from StoryProps to Props (for editable fields)
    // and from Props to StoryProps (for serialization)
    // For Signal types, read the signal value
    let props_to_story_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            if is_non_serializable_type(field_ty) {
                quote! { #field_name: () }
            } else if is_editable_signal_type(field_ty) {
                // For Signal<T>, read the value from the signal
                quote! { #field_name: props.#field_name.read().clone() }
            } else {
                quote! { #field_name: props.#field_name.clone() }
            }
        })
        .collect();

    // For Signal types, wrap the value in a new Signal
    let story_to_props_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            if is_non_serializable_type(field_ty) {
                // Use the default value from StorybookDefault for non-serializable fields
                quote! { #field_name: default_props.#field_name.clone() }
            } else if is_editable_signal_type(field_ty) {
                // For Signal<T>, wrap the value in a new Signal and convert to expected type
                quote! { #field_name: dioxus::prelude::Signal::new(story_props.#field_name.clone()).into() }
            } else {
                quote! { #field_name: story_props.#field_name.clone() }
            }
        })
        .collect();

    let expanded = quote! {
        #input

        /// Auto-generated story props struct for storybook UI editing.
        /// Non-serializable fields (EventHandler, Callback, Element, etc.) are mapped to ().
        #[derive(Clone, storybook::serde::Serialize, storybook::serde::Deserialize, storybook::schemars::JsonSchema)]
        #[doc(hidden)]
        pub struct #story_props_name {
            #(#story_props_fields),*
        }

        impl #story_props_name {
            /// Convert from the original Props to StoryProps
            pub fn from_props(props: &#struct_name) -> Self {
                Self {
                    #(#props_to_story_fields),*
                }
            }

            /// Convert StoryProps back to Props, using defaults for non-serializable fields
            pub fn to_props(&self, default_props: &#struct_name) -> #struct_name {
                let story_props = self;
                #struct_name {
                    #(#story_to_props_fields),*
                }
            }
        }

        #[doc(hidden)]
        fn #render_fn_name(props_json: &str) -> storybook::dioxus::prelude::Element {
            use storybook::dioxus::prelude::*;
            use storybook::Stories;

            let stories = <#struct_name as Stories>::stories();
            let default_props = stories.into_iter().next().expect("At least one story must be defined").props;

            // Try to parse the JSON, fall back to defaults on error
            let props = match storybook::serde_json::from_str::<#story_props_name>(props_json) {
                Ok(story_props) => story_props.to_props(&default_props),
                Err(_) => default_props,
            };

            rsx! {
                #component_name { ..props }
            }
        }

        #[doc(hidden)]
        fn #get_stories_fn_name() -> Vec<storybook::StoryInfo> {
            use storybook::Stories;
            <#struct_name as Stories>::stories()
                .into_iter()
                .map(|story| {
                    let story_props = #story_props_name::from_props(&story.props);
                    storybook::StoryInfo {
                        title: story.title.to_string(),
                        description: story.description.map(|d| d.to_string()),
                        props_json: storybook::serde_json::to_string_pretty(&story_props).unwrap_or_default(),
                    }
                })
                .collect()
        }

        #[doc(hidden)]
        fn #get_prop_fields_fn_name() -> Vec<storybook::PropFieldInfo> {
            vec![
                #(#field_infos),*
            ]
        }

        #[doc(hidden)]
        fn #get_prop_schema_fn_name() -> storybook::schemars::schema::RootSchema {
            storybook::schemars::schema_for!(#story_props_name)
        }

        storybook::inventory::submit! {
            storybook::ComponentRegistration {
                name: #component_name_str,
                tag: #tag,
                render_with_props: #render_fn_name,
                get_stories: #get_stories_fn_name,
                get_prop_fields: #get_prop_fields_fn_name,
                get_prop_schema: #get_prop_schema_fn_name,
            }
        }
    };

    TokenStream::from(expanded)
}

fn storybook_for_function(input: ItemFn, attr_args: StorybookArgs) -> TokenStream {
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let tag = &attr_args.tag;

    // Check if this is a props struct pattern (single argument named "props" with a type ending in "Props")
    let is_props_struct = is_props_struct_pattern(&input);

    let expanded = if is_props_struct {
        // For props struct pattern, the storybook attribute should be on the Props struct instead
        // Generate a no-op registration that won't conflict
        quote! {
            #input
        }
    } else {
        // Generate props struct name (Dioxus generates ComponentNameProps)
        let props_struct_name = format_ident!("{}Props", fn_name_str);
        let story_props_name = format_ident!("{}StoryProps", fn_name_str);
        let render_fn_name =
            format_ident!("__storybook_render_with_props_{}", fn_name_str.to_lowercase());
        let get_stories_fn_name =
            format_ident!("__storybook_get_stories_{}", fn_name_str.to_lowercase());
        let get_prop_fields_fn_name =
            format_ident!("__storybook_get_prop_fields_{}", fn_name_str.to_lowercase());
        let get_prop_schema_fn_name =
            format_ident!("__storybook_get_prop_schema_{}", fn_name_str.to_lowercase());

        // Extract function parameters (these become the props struct fields)
        let params: Vec<_> = input
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        let name = &pat_ident.ident;
                        let ty = &*pat_type.ty;
                        return Some((name.clone(), ty.clone()));
                    }
                }
                None
            })
            .collect();

        // Generate StoryProps fields - map non-serializable types to ()
        // For Signal types, use the inner type instead of the full Signal type
        let story_props_fields: Vec<_> = params
            .iter()
            .map(|(name, ty)| {
                if is_non_serializable_type(ty) {
                    quote! { pub #name: () }
                } else if is_editable_signal_type(ty) {
                    // For Signal<T>, ReadSignal<T>, WriteSignal<T>, use the inner type T
                    if let Some(inner_type_str) = extract_signal_inner_type_str(ty) {
                        let inner_ty: syn::Type = syn::parse_str(&inner_type_str).unwrap();
                        quote! { pub #name: #inner_ty }
                    } else {
                        quote! { pub #name: #ty }
                    }
                } else {
                    quote! { pub #name: #ty }
                }
            })
            .collect();

        // Generate field info for the UI
        let field_infos: Vec<_> = params
            .iter()
            .map(|(name, ty)| {
                let field_name = name.to_string();
                let editable = !is_non_serializable_type(ty);
                let type_name = get_type_display_name(ty);
                quote! {
                    storybook::PropFieldInfo {
                        name: #field_name,
                        editable: #editable,
                        type_name: #type_name,
                    }
                }
            })
            .collect();

        // Generate conversion from Props to StoryProps (for serialization)
        // For Signal types, read the signal value
        let props_to_story_fields: Vec<_> = params
            .iter()
            .map(|(name, ty)| {
                if is_non_serializable_type(ty) {
                    quote! { #name: () }
                } else if is_editable_signal_type(ty) {
                    // For Signal types, read the current value
                    quote! { #name: props.#name.read().clone() }
                } else {
                    quote! { #name: props.#name.clone() }
                }
            })
            .collect();

        // Generate conversion from StoryProps to Props
        // For Signal types, wrap the value in a new Signal
        let story_to_props_fields: Vec<_> = params
            .iter()
            .map(|(name, ty)| {
                if is_non_serializable_type(ty) {
                    // Use the default value from StorybookDefault for non-serializable fields
                    quote! { #name: default_props.#name.clone() }
                } else if is_editable_signal_type(ty) {
                    // For Signal types, wrap the value in a new Signal and convert to expected type
                    quote! { #name: dioxus::prelude::Signal::new(story_props.#name.clone()).into() }
                } else {
                    quote! { #name: story_props.#name.clone() }
                }
            })
            .collect();

        quote! {
            #input

            /// Auto-generated story props struct for storybook UI editing.
            /// Non-serializable fields (EventHandler, Callback, Element, etc.) are mapped to ().
            #[derive(Clone, storybook::serde::Serialize, storybook::serde::Deserialize, storybook::schemars::JsonSchema)]
            #[doc(hidden)]
            pub struct #story_props_name {
                #(#story_props_fields),*
            }

            impl #story_props_name {
                /// Convert from the original Props to StoryProps
                pub fn from_props(props: &#props_struct_name) -> Self {
                    Self {
                        #(#props_to_story_fields),*
                    }
                }

                /// Convert StoryProps back to Props, using defaults for non-serializable fields
                pub fn to_props(&self, default_props: &#props_struct_name) -> #props_struct_name {
                    let story_props = self;
                    #props_struct_name {
                        #(#story_to_props_fields),*
                    }
                }
            }

            #[doc(hidden)]
            fn #render_fn_name(props_json: &str) -> storybook::dioxus::prelude::Element {
                use storybook::dioxus::prelude::*;
                use storybook::Stories;

                let stories = <#props_struct_name as Stories>::stories();
                let default_props = stories.into_iter().next().expect("At least one story must be defined").props;

                // Try to parse the JSON, fall back to defaults on error
                let props = match storybook::serde_json::from_str::<#story_props_name>(props_json) {
                    Ok(story_props) => story_props.to_props(&default_props),
                    Err(_) => default_props,
                };

                rsx! {
                    #fn_name { ..props }
                }
            }

            #[doc(hidden)]
            fn #get_stories_fn_name() -> Vec<storybook::StoryInfo> {
                use storybook::Stories;
                <#props_struct_name as Stories>::stories()
                    .into_iter()
                    .map(|story| {
                        let story_props = #story_props_name::from_props(&story.props);
                        storybook::StoryInfo {
                            title: story.title.to_string(),
                            description: story.description.map(|d| d.to_string()),
                            props_json: storybook::serde_json::to_string_pretty(&story_props).unwrap_or_default(),
                        }
                    })
                    .collect()
            }

            #[doc(hidden)]
            fn #get_prop_fields_fn_name() -> Vec<storybook::PropFieldInfo> {
                vec![
                    #(#field_infos),*
                ]
            }

            #[doc(hidden)]
            fn #get_prop_schema_fn_name() -> storybook::schemars::schema::RootSchema {
                storybook::schemars::schema_for!(#story_props_name)
            }

            storybook::inventory::submit! {
                storybook::ComponentRegistration {
                    name: #fn_name_str,
                    tag: #tag,
                    render_with_props: #render_fn_name,
                    get_stories: #get_stories_fn_name,
                    get_prop_fields: #get_prop_fields_fn_name,
                    get_prop_schema: #get_prop_schema_fn_name,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Check if the function uses a props struct pattern (single argument named "props" with a type ending in "Props")
fn is_props_struct_pattern(input: &ItemFn) -> bool {
    let args: Vec<_> = input.sig.inputs.iter().collect();
    if args.len() == 1 {
        if let FnArg::Typed(pat_type) = &args[0] {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                if pat_ident.ident == "props" {
                    // Check if the type name ends with "Props"
                    let ty = &*pat_type.ty;
                    let ty_str = quote!(#ty).to_string().replace(" ", "");
                    return ty_str.ends_with("Props");
                }
            }
        }
    }
    false
}

struct StorybookArgs {
    tag: String,
}

impl syn::parse::Parse for StorybookArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tag = String::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "tag" {
                let _: syn::Token![=] = input.parse()?;
                let lit: syn::LitStr = input.parse()?;
                tag = lit.value();
            }

            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(StorybookArgs { tag })
    }
}
