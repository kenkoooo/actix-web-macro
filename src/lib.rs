use proc_macro::TokenStream;
use syn::{parse_macro_input, Attribute, ItemFn};

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ret = item.clone();
    let ast = parse_macro_input!(item as ItemFn);

    parse_path_macros(&ast.attrs);
    ret
}

fn parse_path_macros(attrs: &[Attribute]) {
    for attr in attrs {
        parse_path_macro(attr);
    }
}

fn parse_path_macro(attr: &Attribute) -> Option<()> {
    let segments = &attr.path.segments;
    let method = match segments.len() {
        1 => segments[0].ident.to_string(),
        2 => {
            if segments[0].ident.to_string() != "actix_web" {
                return None;
            }
            segments[1].ident.to_string()
        }
        _ => {
            return None;
        }
    };

    if !matches!(
        method.as_str(),
        "get" | "post" | "put" | "delete" | "head" | "connect" | "options" | "trace" | "patch"
    ) {
        return None;
    }

    None
}

struct PathMacro {
    method: String,
}
