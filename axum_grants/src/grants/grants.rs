use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};
use crate::grants::grants_info::GrantInfo;

pub(crate) fn expand_protect(attr: TokenStream, input: TokenStream, is_cus: bool) -> TokenStream {
    let mut item_fn = parse_macro_input!(input as ItemFn);
    match do_protect_expand(attr, &mut item_fn, is_cus) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_protect_expand(
    attr: TokenStream,
    item_fn: &mut ItemFn,
    is_cus: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    // parse grant info from attributes
    let grants_info = GrantInfo::parse(attr.to_string().as_str()).ok_or(syn::Error::new_spanned(
        item_fn.clone(),
        "Incorrect parameter format",
    ))?;

    // build code string
    let code_str = match grants_info.operator {
        0 => build_equal_code(
            grants_info
                .perms
                .first()
                .unwrap_or(&"".to_string())
                .to_string(),
            is_cus,
        ),
        1 => build_any_code(grants_info.perms, is_cus),
        _ => build_all_code(grants_info.perms, is_cus),
    };

    let code_block: syn::Block = syn::parse_str(code_str.as_str())?;
    let mut code_block_stmts: Vec<syn::Stmt> = code_block.stmts;

    // insert code snippet into the function body
    let fn_block = &item_fn.block;
    if !fn_block.stmts.is_empty() {
        code_block_stmts.extend(fn_block.stmts.clone());
    }
    item_fn.block.stmts = code_block_stmts;

    Ok(quote::quote!(#item_fn))
}

fn build_into_response_code(is_cus: bool, msg: String) -> String {
    if is_cus {
        return format!(r#"return AxumGrantsResponse::get_into_response("{}").into_response();"#, msg.replace("\"", "'"));
    }

    format!(
        r#"  return axum::http::Response::builder()
                .status(axum::http::StatusCode::FORBIDDEN)
                .body(Body::from("{}"))
                .unwrap();"#,
        msg.replace("\"", "'")
    )
}

fn build_equal_code(perm: String, is_cus: bool) -> String {
    format!(
        r#"    {{
        if !claims.perms.contains(&"{}".to_string()) {{
           {}
        }}
    }}"#,
        perm,
        build_into_response_code(
            is_cus,
            format!("Insufficient permissions, need the permission: {} ", perm)
        )
    )
}

fn build_any_code(perms: Vec<String>, is_cus: bool) -> String {
    let perms_str = perms
        .iter()
        .map(|perm| format!("\"{}\"", perm)) // 每个元素前后加上双引号
        .collect::<Vec<_>>() // 转换为Vec<String>
        .join(",");

    format!(
        r#"    {{
        let resource_perms = vec![{}];
        if !resource_perms
            .iter()
            .any(|perm| claims.perms.contains(&perm.to_string()))
        {{
            {}
        }}
    }}"#,
        perms_str,
        build_into_response_code(
            is_cus,
            format!(
                "Insufficient permissions, need any permission: {}",
                perms_str
            )
        )
    )
}

fn build_all_code(perms: Vec<String>, is_cus: bool) -> String {
    let perms_str = perms
        .iter()
        .map(|perm| format!("\"{}\"", perm)) // 每个元素前后加上双引号
        .collect::<Vec<_>>() // 转换为Vec<String>
        .join(",");

    format!(
        r#"    {{
        let resource_perms = vec![{}];
        if !resource_perms
            .iter()
            .all(|perm| claims.perms.contains(&perm.to_string()))
        {{
            {}
        }}
    }}"#,
        perms_str,
        build_into_response_code(
            is_cus,
            format!(
                "Insufficient permissions, need all permission: {}",
                perms_str
            )
        )
    )
}