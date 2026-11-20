use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::fmt::Write as _;

#[derive(Clone, Copy, PartialEq, Eq)]
enum LayoutKind {
    Std140,
    Std430,
    Scalar,
}

impl LayoutKind {
    fn trait_name(self) -> &'static str {
        match self {
            Self::Std140 => "Std140",
            Self::Std430 => "Std430",
            Self::Scalar => "Scalar",
        }
    }

    fn suffix(self) -> &'static str {
        self.trait_name()
    }

    fn to_method(self) -> &'static str {
        match self {
            Self::Std140 => "to_std140",
            Self::Std430 => "to_std430",
            Self::Scalar => "to_scalar",
        }
    }

    fn write_method(self) -> &'static str {
        match self {
            Self::Std140 => "write_std140",
            Self::Std430 => "write_std430",
            Self::Scalar => "write_scalar",
        }
    }

    fn attr_key(self) -> &'static str {
        match self {
            Self::Std140 => "std140",
            Self::Std430 => "std430",
            Self::Scalar => "scalar",
        }
    }
}

struct ParsedField {
    vis: String,
    name: String,
    ty: String,
}

struct ParsedStruct {
    vis: String,
    name: String,
    crate_path: String,
    custom_repr: Option<String>,
    fields: Vec<ParsedField>,
}

/// Derives a Vulkan `std140` uniform buffer layout representation and `rglm::layout::Std140` implementation.
#[proc_macro_derive(Std140, attributes(rglm))]
pub fn derive_std140(input: TokenStream) -> TokenStream {
    expand_layout(input, LayoutKind::Std140)
}

/// Derives a Vulkan `std430` storage buffer layout representation and `rglm::layout::Std430` implementation.
#[proc_macro_derive(Std430, attributes(rglm))]
pub fn derive_std430(input: TokenStream) -> TokenStream {
    expand_layout(input, LayoutKind::Std430)
}

/// Derives a Vulkan `VK_EXT_scalar_block_layout` representation and `rglm::layout::Scalar` implementation.
#[proc_macro_derive(Scalar, attributes(rglm))]
pub fn derive_scalar(input: TokenStream) -> TokenStream {
    expand_layout(input, LayoutKind::Scalar)
}

fn compile_error_ts(msg: &str) -> TokenStream {
    let escaped = msg.escape_default().to_string();
    format!("::core::compile_error!(\"{escaped}\");")
        .parse()
        .unwrap_or_else(|_| TokenStream::new())
}

fn tokens_to_string(tokens: &[TokenTree]) -> String {
    let mut s = String::new();
    for tt in tokens {
        if !s.is_empty() {
            s.push(' ');
        }
        s.push_str(&tt.to_string());
    }
    s
}

fn strip_string_quotes(lit: &str) -> Option<String> {
    if lit.len() >= 2 && lit.starts_with('"') && lit.ends_with('"') {
        Some(lit[1..lit.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_rglm_attr(
    group: &proc_macro::Group,
    kind: LayoutKind,
    crate_path: &mut String,
    custom_repr: &mut Option<String>,
) -> Result<(), &'static str> {
    let inner: Vec<TokenTree> = group.stream().into_iter().collect();
    if inner.is_empty() {
        return Ok(());
    }
    let TokenTree::Ident(head) = &inner[0] else {
        return Ok(());
    };
    if head.to_string() != "rglm" {
        return Ok(());
    }
    if inner.len() < 2 {
        return Err("expected parenthesized arguments in #[rglm(...)]");
    }
    let TokenTree::Group(args_grp) = &inner[1] else {
        return Err("expected parenthesized arguments in #[rglm(...)]");
    };
    if args_grp.delimiter() != Delimiter::Parenthesis {
        return Err("expected parenthesized arguments in #[rglm(...)]");
    }

    let args: Vec<TokenTree> = args_grp.stream().into_iter().collect();
    let mut idx = 0usize;
    while idx < args.len() {
        let TokenTree::Ident(key_ident) = &args[idx] else {
            return Err("expected key identifier in #[rglm(...)]");
        };
        let key = key_ident.to_string();
        idx += 1;
        if idx >= args.len() {
            return Err("expected `=` after key in #[rglm(...)]");
        }
        let TokenTree::Punct(eq_p) = &args[idx] else {
            return Err("expected `=` after key in #[rglm(...)]");
        };
        if eq_p.as_char() != '=' {
            return Err("expected `=` after key in #[rglm(...)]");
        }
        idx += 1;
        if idx >= args.len() {
            return Err("expected value after `=` in #[rglm(...)]");
        }
        let val_str = match &args[idx] {
            TokenTree::Ident(id) => id.to_string(),
            TokenTree::Literal(lit) => {
                strip_string_quotes(&lit.to_string()).ok_or("expected string or ident value")?
            }
            _ => return Err("expected identifier or string literal in #[rglm(...)]"),
        };
        idx += 1;

        if key == "crate" {
            *crate_path = val_str;
        } else if key == kind.attr_key() {
            *custom_repr = Some(val_str);
        }

        if idx < args.len() {
            let TokenTree::Punct(comma) = &args[idx] else {
                return Err("expected `,` between #[rglm(...)] arguments");
            };
            if comma.as_char() != ',' {
                return Err("expected `,` between #[rglm(...)] arguments");
            }
            idx += 1;
        }
    }
    Ok(())
}

fn parse_vis(tokens: &[TokenTree], idx: &mut usize) -> String {
    if *idx < tokens.len()
        && let TokenTree::Ident(id) = &tokens[*idx]
        && id.to_string() == "pub"
    {
        *idx += 1;
        if *idx < tokens.len()
            && let TokenTree::Group(g) = &tokens[*idx]
            && g.delimiter() == Delimiter::Parenthesis
        {
            let vis = format!("pub{}", g);
            *idx += 1;
            return vis;
        }
        return String::from("pub");
    }
    String::new()
}

type AttrCallback<'a> = dyn FnMut(&proc_macro::Group) -> Result<(), &'static str> + 'a;

fn skip_attrs(
    tokens: &[TokenTree],
    idx: &mut usize,
    mut on_attr: Option<&mut AttrCallback<'_>>,
) -> Result<(), &'static str> {
    while *idx < tokens.len() {
        if let TokenTree::Punct(p) = &tokens[*idx]
            && p.as_char() == '#'
        {
            if *idx + 1 >= tokens.len() {
                return Err("unexpected end of attribute");
            }
            let TokenTree::Group(g) = &tokens[*idx + 1] else {
                return Err("malformed attribute");
            };
            if g.delimiter() != Delimiter::Bracket {
                return Err("malformed attribute bracket");
            }
            if let Some(cb) = on_attr.as_deref_mut() {
                cb(g)?;
            }
            *idx += 2;
        } else {
            break;
        }
    }
    Ok(())
}

fn parse_fields(body: TokenStream) -> Result<Vec<ParsedField>, &'static str> {
    let tokens: Vec<TokenTree> = body.into_iter().collect();
    let mut idx = 0usize;
    let mut fields = Vec::new();

    while idx < tokens.len() {
        skip_attrs(&tokens, &mut idx, None)?;
        if idx >= tokens.len() {
            break;
        }

        let vis = parse_vis(&tokens, &mut idx);
        if idx >= tokens.len() {
            return Err("expected field name");
        }
        let TokenTree::Ident(field_ident) = &tokens[idx] else {
            return Err("expected named struct field");
        };
        let name = field_ident.to_string();
        idx += 1;

        if idx >= tokens.len() {
            return Err("expected `:` after field name");
        }
        let TokenTree::Punct(colon) = &tokens[idx] else {
            return Err("expected `:` after field name");
        };
        if colon.as_char() != ':' {
            return Err("expected `:` after field name");
        }
        idx += 1;

        let ty_start = idx;
        let mut angle_depth = 0i32;
        while idx < tokens.len() {
            if let TokenTree::Punct(p) = &tokens[idx] {
                match p.as_char() {
                    '<' => angle_depth += 1,
                    '>' => {
                        if angle_depth > 0 {
                            angle_depth -= 1;
                        }
                    }
                    ',' if angle_depth == 0 => break,
                    _ => {}
                }
            }
            idx += 1;
        }

        if ty_start == idx {
            return Err("expected field type");
        }
        let ty = tokens_to_string(&tokens[ty_start..idx]);
        fields.push(ParsedField { vis, name, ty });

        if idx < tokens.len()
            && let TokenTree::Punct(p) = &tokens[idx]
            && p.as_char() == ','
        {
            idx += 1;
        }
    }

    Ok(fields)
}

fn parse_struct(input: TokenStream, kind: LayoutKind) -> Result<ParsedStruct, &'static str> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut idx = 0usize;
    let mut crate_path = String::from("::rglm");
    let mut custom_repr = None;

    {
        let mut attr_cb =
            |g: &proc_macro::Group| parse_rglm_attr(g, kind, &mut crate_path, &mut custom_repr);
        skip_attrs(&tokens, &mut idx, Some(&mut attr_cb))?;
    }

    let vis = parse_vis(&tokens, &mut idx);

    if idx >= tokens.len() {
        return Err("expected `struct` keyword");
    }
    let TokenTree::Ident(kw) = &tokens[idx] else {
        return Err("expected `struct` item");
    };
    if kw.to_string() != "struct" {
        return Err("Std140, Std430, and Scalar can only be derived on structs");
    }
    idx += 1;

    if idx >= tokens.len() {
        return Err("expected struct identifier");
    }
    let TokenTree::Ident(name_ident) = &tokens[idx] else {
        return Err("expected struct identifier");
    };
    let name = name_ident.to_string();
    idx += 1;

    if idx < tokens.len()
        && let TokenTree::Punct(p) = &tokens[idx]
        && p.as_char() == '<'
    {
        return Err("generic parameters are not supported on layout-derived structs");
    }

    if idx >= tokens.len() {
        return Err("expected braced struct body");
    }
    let TokenTree::Group(body_grp) = &tokens[idx] else {
        return Err("tuple and unit structs are not supported; use named fields");
    };
    if body_grp.delimiter() != Delimiter::Brace {
        return Err("tuple structs are not supported; use named fields");
    }

    let fields = parse_fields(body_grp.stream())?;
    Ok(ParsedStruct {
        vis,
        name,
        crate_path,
        custom_repr,
        fields,
    })
}

fn expand_layout(input: TokenStream, kind: LayoutKind) -> TokenStream {
    let parsed = match parse_struct(input, kind) {
        Ok(s) => s,
        Err(msg) => return compile_error_ts(msg),
    };

    let vis = if parsed.vis.is_empty() {
        String::new()
    } else {
        format!("{} ", parsed.vis)
    };
    let raw_name = &parsed.name;
    let clean_name = raw_name.strip_prefix("r#").unwrap_or(raw_name);
    let repr_name = parsed
        .custom_repr
        .unwrap_or_else(|| format!("{clean_name}{}", kind.suffix()));
    let align_struct_name = format!("{repr_name}Align");
    let krate = &parsed.crate_path;
    let trait_path = format!("{krate}::layout::{}", kind.trait_name());
    let layout_zero = format!("{krate}::layout::LayoutZero");
    let align_fn = format!("{krate}::layout::align_offset");
    let max_fn = format!("{krate}::layout::max_usize");
    let to_method = kind.to_method();
    let write_method = kind.write_method();

    let mut code = String::new();

    if kind == LayoutKind::Std140 {
        let _ = writeln!(code, "#[derive(Clone, Copy, Debug)]");
        let _ = writeln!(code, "#[repr(C, align(16))]");
    } else {
        let _ = writeln!(code, "#[derive(Clone, Copy, Debug)]");
        let _ = writeln!(code, "#[repr(C)]");
    }
    let _ = writeln!(code, "{vis}struct {align_struct_name}(");
    if parsed.fields.is_empty() {
        let _ = writeln!(code, "    pub [{krate}::layout::Align1; 0],");
    } else {
        for (i, f) in parsed.fields.iter().enumerate() {
            let _ = writeln!(code, "    pub [<{} as {trait_path}>::Align; 0],", f.ty);
            let _ = i;
        }
    }
    let _ = writeln!(code, ");");

    let _ = writeln!(code, "impl {layout_zero} for {align_struct_name} {{");
    let _ = write!(code, "    const ZERO_REPR: Self = Self(");
    if parsed.fields.is_empty() {
        let _ = write!(code, "[]");
    } else {
        for idx in 0..parsed.fields.len() {
            if idx > 0 {
                let _ = write!(code, ", ");
            }
            let _ = write!(code, "[]");
        }
    }
    let _ = writeln!(code, ");");
    let _ = writeln!(code, "}}");

    let _ = writeln!(code, "#[derive(Clone, Copy, Debug)]");
    let _ = writeln!(code, "#[repr(C)]");
    let _ = writeln!(code, "{vis}struct {repr_name} {{");
    if parsed.fields.is_empty() && kind != LayoutKind::Std140 {
        let _ = writeln!(code, "    pub _rglm_empty: {krate}::layout::Align1,");
    }
    for (idx, f) in parsed.fields.iter().enumerate() {
        let fvis = if f.vis.is_empty() {
            String::new()
        } else {
            format!("{} ", f.vis)
        };
        let _ = writeln!(
            code,
            "    pub _rglm_align_{idx}: <{} as {trait_path}>::Align,",
            f.ty
        );
        let _ = writeln!(
            code,
            "    {fvis}{}: <{} as {trait_path}>::Repr,",
            f.name, f.ty
        );
    }
    if kind == LayoutKind::Std140 {
        let _ = writeln!(code, "    pub _rglm_std140_end: {krate}::layout::Align16,");
    }
    let _ = writeln!(code, "}}");

    let _ = writeln!(code, "impl {layout_zero} for {repr_name} {{");
    let _ = writeln!(code, "    const ZERO_REPR: Self = Self {{");
    if parsed.fields.is_empty() && kind != LayoutKind::Std140 {
        let _ = writeln!(
            code,
            "        _rglm_empty: <{krate}::layout::Align1 as {layout_zero}>::ZERO_REPR,"
        );
    }
    for (idx, f) in parsed.fields.iter().enumerate() {
        let _ = writeln!(
            code,
            "        _rglm_align_{idx}: <<{} as {trait_path}>::Align as {layout_zero}>::ZERO_REPR,",
            f.ty
        );
        let _ = writeln!(
            code,
            "        {}: <<{} as {trait_path}>::Repr as {layout_zero}>::ZERO_REPR,",
            f.name, f.ty
        );
    }
    if kind == LayoutKind::Std140 {
        let _ = writeln!(
            code,
            "        _rglm_std140_end: <{krate}::layout::Align16 as {layout_zero}>::ZERO_REPR,"
        );
    }
    let _ = writeln!(code, "    }};");
    let _ = writeln!(code, "}}");

    let _ = writeln!(
        code,
        "impl ::core::convert::From<&{raw_name}> for {repr_name} {{"
    );
    let _ = writeln!(code, "    #[inline]");
    let _ = writeln!(code, "    fn from(src: &{raw_name}) -> Self {{");
    let _ = writeln!(
        code,
        "        <{raw_name} as {trait_path}>::{to_method}(src)"
    );
    let _ = writeln!(code, "    }}");
    let _ = writeln!(code, "}}");

    let _ = writeln!(
        code,
        "impl ::core::convert::From<{raw_name}> for {repr_name} {{"
    );
    let _ = writeln!(code, "    #[inline]");
    let _ = writeln!(code, "    fn from(src: {raw_name}) -> Self {{");
    let _ = writeln!(
        code,
        "        <{raw_name} as {trait_path}>::{to_method}(&src)"
    );
    let _ = writeln!(code, "    }}");
    let _ = writeln!(code, "}}");

    let init_align = if kind == LayoutKind::Std140 {
        16usize
    } else {
        1usize
    };
    let _ = writeln!(code, "impl {trait_path} for {raw_name} {{");
    let _ = writeln!(code, "    const ALIGN: usize = {{");
    let _ = writeln!(code, "        let mut a: usize = {init_align};");
    for f in &parsed.fields {
        let _ = writeln!(
            code,
            "        a = {max_fn}(a, <{} as {trait_path}>::ALIGN);",
            f.ty
        );
    }
    let _ = writeln!(code, "        a");
    let _ = writeln!(code, "    }};");

    let _ = writeln!(code, "    const SIZE: usize = {{");
    let _ = writeln!(code, "        let mut off: usize = 0;");
    for f in &parsed.fields {
        let _ = writeln!(
            code,
            "        off = {align_fn}(off, <{} as {trait_path}>::ALIGN);",
            f.ty
        );
        let _ = writeln!(code, "        off += <{} as {trait_path}>::SIZE;", f.ty);
    }
    let _ = writeln!(
        code,
        "        {align_fn}(off, <Self as {trait_path}>::ALIGN)"
    );
    let _ = writeln!(code, "    }};");

    let _ = writeln!(code, "    type Align = {align_struct_name};");
    let _ = writeln!(code, "    type Repr = {repr_name};");

    let _ = writeln!(code, "    #[inline]");
    let _ = writeln!(code, "    fn {to_method}(&self) -> Self::Repr {{");
    let _ = writeln!(code, "        {repr_name} {{");
    if parsed.fields.is_empty() && kind != LayoutKind::Std140 {
        let _ = writeln!(
            code,
            "            _rglm_empty: <{krate}::layout::Align1 as {layout_zero}>::ZERO_REPR,"
        );
    }
    for (idx, f) in parsed.fields.iter().enumerate() {
        let _ = writeln!(
            code,
            "            _rglm_align_{idx}: <<{} as {trait_path}>::Align as {layout_zero}>::ZERO_REPR,",
            f.ty
        );
        let _ = writeln!(
            code,
            "            {}: <{} as {trait_path}>::{to_method}(&self.{}),",
            f.name, f.ty, f.name
        );
    }
    if kind == LayoutKind::Std140 {
        let _ = writeln!(
            code,
            "            _rglm_std140_end: <{krate}::layout::Align16 as {layout_zero}>::ZERO_REPR,"
        );
    }
    let _ = writeln!(code, "        }}");
    let _ = writeln!(code, "    }}");

    let _ = writeln!(code, "    #[inline]");
    let _ = writeln!(code, "    fn {write_method}(&self, dst: &mut [u8]) {{");
    let _ = writeln!(code, "        dst[..<Self as {trait_path}>::SIZE].fill(0);");
    for f in &parsed.fields {
        let _ = writeln!(code, "        {{");
        let _ = writeln!(
            code,
            "            let off = ::core::mem::offset_of!({repr_name}, {});",
            f.name
        );
        let _ = writeln!(
            code,
            "            let sz = <{} as {trait_path}>::SIZE;",
            f.ty
        );
        let _ = writeln!(
            code,
            "            <{} as {trait_path}>::{write_method}(&self.{}, &mut dst[off..off + sz]);",
            f.ty, f.name
        );
        let _ = writeln!(code, "        }}");
    }
    let _ = writeln!(code, "    }}");
    let _ = writeln!(code, "}}");

    let _ = writeln!(code, "const _: () = {{");
    let _ = writeln!(code, "    let mut expected: usize = 0;");
    for f in &parsed.fields {
        let _ = writeln!(
            code,
            "    expected = {align_fn}(expected, <{} as {trait_path}>::ALIGN);",
            f.ty
        );
        let _ = writeln!(
            code,
            "    assert!(::core::mem::offset_of!({repr_name}, {}) == expected);",
            f.name
        );
        let _ = writeln!(code, "    expected += <{} as {trait_path}>::SIZE;", f.ty);
    }
    let _ = writeln!(code, "    let _ = expected;");
    let _ = writeln!(
        code,
        "    assert!(::core::mem::align_of::<{repr_name}>() == <{raw_name} as {trait_path}>::ALIGN);"
    );
    let _ = writeln!(
        code,
        "    assert!(::core::mem::size_of::<{repr_name}>() == <{raw_name} as {trait_path}>::SIZE);"
    );
    let _ = writeln!(code, "}};");

    code.parse().unwrap_or_else(|_| {
        compile_error_ts("internal error generating Vulkan layout implementation")
    })
}
