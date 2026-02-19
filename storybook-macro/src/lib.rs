use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use pulldown_cmark::{Options, Parser, html};
use quote::{format_ident, quote};
use syn::{Fields, FnArg, Ident, ItemFn, ItemStruct, Pat, Type, parse_macro_input};

/// Common field information used by both struct and function storybook processing
struct FieldInfo {
    name: Ident,
    ty: Type,
    /// Doc comments for this field (to be preserved in generated StoryProps)
    doc_attrs: Vec<syn::Attribute>,
}

/// Metadata about the component being processed
struct ComponentMeta {
    component_name: Ident,
    component_name_str: String,
    props_struct_name: Ident,
    story_props_name: Ident,
    tag: String,
    /// HTML description extracted from doc comments
    description_html: String,
}

/// Extract doc comments from a list of attributes and return them as a single string
fn extract_doc_comments(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                // Parse the doc attribute to extract the string content
                if let syn::Meta::NameValue(meta) = &attr.meta
                    && let syn::Expr::Lit(expr_lit) = &meta.value
                    && let syn::Lit::Str(lit_str) = &expr_lit.lit
                {
                    return Some(lit_str.value());
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Convert Markdown text to HTML using pulldown-cmark.
///
/// When `process_story_embeds` is `true`, the raw markdown is pre-processed
/// to replace `@[story:Category/Component/Story Name]` lines with
/// `<div class="storybook-embed" …></div>` HTML blocks before parsing.
/// This avoids issues with pulldown-cmark splitting `@[story:…]` across
/// multiple text events.
fn markdown_to_html(markdown: &str, process_story_embeds: bool) -> String {
    let source = if process_story_embeds {
        preprocess_story_embeds(markdown)
    } else {
        markdown.to_string()
    };

    let options = Options::all();
    let parser = Parser::new_ext(&source, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Pre-process raw markdown to replace `@[story:…]` lines with HTML embed
/// markers before feeding the text to pulldown-cmark.
///
/// pulldown-cmark treats `[…]` as potential link references and splits the
/// surrounding text across multiple `Text` events, making it impossible to
/// detect `@[story:…]` reliably in the event stream. By replacing matching
/// lines in the source markdown with `<div>` blocks, pulldown-cmark passes
/// them through as native HTML blocks.
fn preprocess_story_embeds(markdown: &str) -> String {
    let mut result = String::with_capacity(markdown.len());
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("@[story:") && trimmed.ends_with(']') {
            let full_path = &trimmed[8..trimmed.len() - 1];
            let story_name = full_path.rsplit('/').next().unwrap_or(full_path);
            result.push_str(&format!(
                "<div class=\"storybook-embed\" data-story-path=\"{}\" data-story-name=\"{}\"></div>\n",
                full_path, story_name
            ));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

impl ComponentMeta {
    fn render_fn_name(&self) -> Ident {
        format_ident!(
            "__storybook_render_with_props_{}",
            self.component_name_str.to_lowercase()
        )
    }

    fn get_stories_fn_name(&self) -> Ident {
        format_ident!(
            "__storybook_get_stories_{}",
            self.component_name_str.to_lowercase()
        )
    }

    fn get_prop_schema_fn_name(&self) -> Ident {
        format_ident!(
            "__storybook_get_prop_schema_{}",
            self.component_name_str.to_lowercase()
        )
    }
}

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

/// Generate StoryProps field definitions from field info
fn generate_story_props_fields(fields: &[FieldInfo]) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ty = &field.ty;
            let doc_attrs = &field.doc_attrs;
            if is_non_serializable_type(ty) {
                quote! {
                    #(#doc_attrs)*
                    pub #name: ()
                }
            } else if let Some(inner_ty_str) = extract_signal_inner_type_str(ty) {
                let inner_ty: Type =
                    syn::parse_str(&inner_ty_str).expect("Failed to parse inner type");
                quote! {
                    #(#doc_attrs)*
                    pub #name: #inner_ty
                }
            } else {
                quote! {
                    #(#doc_attrs)*
                    pub #name: #ty
                }
            }
        })
        .collect()
}

/// Generate Props to StoryProps field conversions
fn generate_props_to_story_fields(fields: &[FieldInfo]) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ty = &field.ty;
            if is_non_serializable_type(ty) {
                quote! { #name: () }
            } else if is_editable_signal_type(ty) {
                quote! { #name: props.#name.read().clone() }
            } else {
                quote! { #name: props.#name.clone() }
            }
        })
        .collect()
}

/// Generate StoryProps to Props field conversions
fn generate_story_to_props_fields(fields: &[FieldInfo]) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ty = &field.ty;
            if is_non_serializable_type(ty) {
                quote! { #name: default_props.#name.clone() }
            } else if is_editable_signal_type(ty) {
                quote! { #name: dioxus::prelude::Signal::new(story_props.#name.clone()).into() }
            } else {
                quote! { #name: story_props.#name.clone() }
            }
        })
        .collect()
}

/// Generate the complete storybook code from metadata and fields
fn generate_storybook_code(
    meta: &ComponentMeta,
    fields: &[FieldInfo],
    original_item: TokenStream2,
) -> TokenStream2 {
    let ComponentMeta {
        component_name,
        component_name_str,
        props_struct_name,
        story_props_name,
        tag,
        description_html,
    } = meta;

    let render_fn_name = meta.render_fn_name();
    let get_stories_fn_name = meta.get_stories_fn_name();
    let get_prop_schema_fn_name = meta.get_prop_schema_fn_name();

    let story_props_fields = generate_story_props_fields(fields);
    let props_to_story_fields = generate_props_to_story_fields(fields);
    let story_to_props_fields = generate_story_to_props_fields(fields);

    quote! {
        #original_item

        /// Auto-generated story props struct for storybook UI editing.
        /// Non-serializable fields (EventHandler, Callback, Element, etc.) are mapped to ().
        #[derive(Clone, storybook::serde::Serialize, storybook::serde::Deserialize, storybook::schemars::JsonSchema)]
        #[serde(crate = "storybook::serde")]
        #[schemars(crate = "storybook::schemars")]
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
                #component_name { ..props }
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
                        decorators: story.decorators,
                    }
                })
                .collect()
        }

        #[doc(hidden)]
        fn #get_prop_schema_fn_name() -> storybook::schemars::Schema {
            storybook::schemars::schema_for!(#story_props_name)
        }

        storybook::inventory::submit! {
            storybook::ComponentRegistration {
                name: #component_name_str,
                tag: #tag,
                description: #description_html,
                render_with_props: storybook::RenderFn(#render_fn_name),
                get_stories: #get_stories_fn_name,
                get_prop_schema: #get_prop_schema_fn_name,
            }
        }
    }
}

fn storybook_for_struct(input: ItemStruct, attr_args: StorybookArgs) -> TokenStream {
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();

    // The struct name should end with "Props", and the component name is without "Props"
    let component_name_str = struct_name_str
        .strip_suffix("Props")
        .unwrap_or(&struct_name_str);

    // Extract doc comments from the struct and convert to HTML
    // process_story_embeds=true so @[story:...] lines become embed markers
    let doc_markdown = extract_doc_comments(&input.attrs);
    let description_html = markdown_to_html(&doc_markdown, true);

    // Extract fields from the struct
    let syn_fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return TokenStream::from(
                syn::Error::new_spanned(
                    &input,
                    "storybook only supports structs with named fields",
                )
                .to_compile_error(),
            );
        }
    };

    // Convert to FieldInfo format, preserving doc comments
    let fields: Vec<FieldInfo> = syn_fields
        .iter()
        .filter_map(|field| {
            field.ident.as_ref().map(|name| {
                // Extract doc attributes from the field
                let doc_attrs: Vec<syn::Attribute> = field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("doc"))
                    .cloned()
                    .collect();
                FieldInfo {
                    name: name.clone(),
                    ty: field.ty.clone(),
                    doc_attrs,
                }
            })
        })
        .collect();

    // Build component metadata
    let meta = ComponentMeta {
        component_name: format_ident!("{}", component_name_str),
        component_name_str: component_name_str.to_string(),
        props_struct_name: struct_name.clone(),
        story_props_name: format_ident!("{}StoryProps", component_name_str),
        tag: attr_args.tag.clone(),
        description_html,
    };

    let original_item = quote! { #input };
    let expanded = generate_storybook_code(&meta, &fields, original_item);

    TokenStream::from(expanded)
}

fn storybook_for_function(input: ItemFn, attr_args: StorybookArgs) -> TokenStream {
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Check if this is a props struct pattern (single argument named "props" with a type ending in "Props")
    // For props struct pattern, the storybook attribute should be on the Props struct instead
    if is_props_struct_pattern(&input) {
        return TokenStream::from(quote! { #input });
    }

    // Extract doc comments from the function and convert to HTML
    // process_story_embeds=true so @[story:...] lines become embed markers
    let doc_markdown = extract_doc_comments(&input.attrs);
    let description_html = markdown_to_html(&doc_markdown, true);

    // Extract function parameters as FieldInfo
    // Note: Function parameters don't have doc comments, so doc_attrs is empty
    let fields: Vec<FieldInfo> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg
                && let Pat::Ident(pat_ident) = &*pat_type.pat
            {
                // Extract doc attributes from the pattern's attributes
                let doc_attrs: Vec<syn::Attribute> = pat_type
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("doc"))
                    .cloned()
                    .collect();
                return Some(FieldInfo {
                    name: pat_ident.ident.clone(),
                    ty: (*pat_type.ty).clone(),
                    doc_attrs,
                });
            }
            None
        })
        .collect();

    // Build component metadata
    let meta = ComponentMeta {
        component_name: fn_name.clone(),
        component_name_str: fn_name_str.clone(),
        props_struct_name: format_ident!("{}Props", fn_name_str),
        story_props_name: format_ident!("{}StoryProps", fn_name_str),
        tag: attr_args.tag,
        description_html,
    };

    let original_item = quote! { #input };
    let expanded = generate_storybook_code(&meta, &fields, original_item);

    TokenStream::from(expanded)
}

/// Check if the function uses a props struct pattern (single argument named "props" with a type ending in "Props")
fn is_props_struct_pattern(input: &ItemFn) -> bool {
    let args: Vec<_> = input.sig.inputs.iter().collect();
    if args.len() == 1
        && let FnArg::Typed(pat_type) = &args[0]
        && let Pat::Ident(pat_ident) = &*pat_type.pat
        && pat_ident.ident == "props"
    {
        // Check if the type name ends with "Props"
        let ty = &*pat_type.ty;
        let ty_str = quote!(#ty).to_string().replace(" ", "");
        return ty_str.ends_with("Props");
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

/// Registers a markdown documentation page for a category or folder in the storybook.
///
/// # Example
/// ```ignore
/// storydoc!("Buttons/Primary", "docs/buttons_primary.md");
/// ```
///
/// The markdown file can embed live story previews using the `@[story:...]` syntax:
/// ```markdown
/// @[story:Category/Component/Story Name]
/// ```
/// This will render the story inline within the documentation.
#[proc_macro]
pub fn storydoc(input: TokenStream) -> TokenStream {
    let parsed = syn::parse::<StorydocArgs2>(input);

    match parsed {
        Ok(args) => {
            let path = args.path;
            let md_file = args.markdown_file;

            // Read the markdown file at compile time
            let manifest_dir =
                std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
            let full_path = std::path::Path::new(&manifest_dir).join(&md_file);

            let markdown_content = match std::fs::read_to_string(&full_path) {
                Ok(content) => content,
                Err(e) => {
                    return TokenStream::from(
                        syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "Failed to read markdown file '{}': {}",
                                full_path.display(),
                                e
                            ),
                        )
                        .to_compile_error(),
                    );
                }
            };

            // Convert markdown to HTML, processing @[story:...] embeds
            let html_content = markdown_to_html(&markdown_content, true);

            // Generate the inventory submission
            let expanded = quote! {
                storybook::inventory::submit! {
                    storybook::DocRegistration {
                        path: #path,
                        content_html: #html_content,
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Err(e) => TokenStream::from(e.to_compile_error()),
    }
}

struct StorydocArgs2 {
    path: String,
    markdown_file: String,
}

impl syn::parse::Parse for StorydocArgs2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let markdown_file: syn::LitStr = input.parse()?;

        Ok(StorydocArgs2 {
            path: path.value(),
            markdown_file: markdown_file.value(),
        })
    }
}
