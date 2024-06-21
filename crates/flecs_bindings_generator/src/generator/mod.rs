use anyhow::{bail, Context, Result};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::{collections::BTreeMap, fmt::Display, str::FromStr};
use syn::{fold::Fold, Expr, ForeignItemFn, Ident, Item, StaticMutability, Type, TypeBareFn};

pub struct GeneratorContext {
    /// By default, we will rename functions by stripping their prefix eg: `ecs_new` becomes `new` inside World struct
    /// This map allows you to override this behavior and specific the new name directly. eg: `ecs_new` -> `create`
    pub explicit_rename_fns: BTreeMap<String, String>,
    /// skip generating bindings for these functions
    /// You will have to manually generate the bindings for these functions yourself (or completely ignore them)
    /// eg: `ecs_new` will not be generated
    pub skip_fns: Vec<String>,
}
impl GeneratorContext {
    pub fn new() -> Self {
        Self {
            explicit_rename_fns: Default::default(),
            skip_fns: Default::default(),
        }
    }
}
pub fn generate_safe_wrappers(source: &str) -> Result<()> {
    let file = syn::parse_file(&source).unwrap();
    let mut gtx = GeneratorContext::new();

    // let file = BindingsTransformer.fold_file(file);
    let mut extern_fns = vec![];
    let mut constants = vec![];
    let mut statics = vec![];
    for item in file.items {
        match item {
            Item::ForeignMod(foreign_mod) => {
                for item in foreign_mod.items {
                    match item {
                        syn::ForeignItem::Fn(foreign_fn) => {
                            let name = foreign_fn.sig.ident.to_string();
                            if (name.len() >= 5)
                                && name[..3].eq_ignore_ascii_case("ecs")
                                && !(name.starts_with("ecs_vec")
                                    || name.starts_with("ecs_map")
                                    || name.starts_with("ecs_array")
                                    || name.starts_with("ecs_sparse")
                                    || name.starts_with("ecs_strbuf")
                                    || name.starts_with("ecs_os_")
                                    || name.starts_with("ecs_trigger_")
                                    || name.starts_with("ecs_type_")
                                    || name.starts_with("ecs_log_")
                                    || name.starts_with("ecs_init_w_args")
                                    || name.starts_with("ecs_log")
                                    || name.ends_with("_"))
                            {
                                match ExternFn::parse_from(&mut gtx, foreign_fn) {
                                    Ok(extern_fn) => {
                                        extern_fns.push(extern_fn);
                                    }
                                    Err(e) => {
                                        println!("failed to parse fn {name}: {e}");
                                    }
                                }
                            } else {
                                println!("ignoring parsing fn {}", name);
                            }
                        }
                        syn::ForeignItem::Static(static_item) => {
                            let name = static_item.ident.to_string();
                            if name.len() >= 5
                                && (name[..3].eq_ignore_ascii_case("ecs")
                                    || name[..5].eq_ignore_ascii_case("flecs"))
                            {
                                statics.push(
                                    ExternStatic::parse_from(static_item).with_context(|| {
                                        format!("failed to parse item: {}", name)
                                    })?,
                                );
                            } else {
                                println!("ignoring static {}", name);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Item::Const(constant) => {
                let name = constant.ident.to_string();

                if name.len() >= 5
                    && (name[..3].eq_ignore_ascii_case("ecs")
                        || name[..5].eq_ignore_ascii_case("flecs"))
                {
                    constants.push(
                        Constant::parse_from(constant)
                            .with_context(|| format!("failed to parse item: {}", name))?,
                    );
                } else {
                    println!("ignoring constant {}", name);
                }
            }
            _ => {}
        }
    }
    let mut stream = TokenStream::new();

    for constant in constants {
        let name = Ident::new(&constant.name, proc_macro2::Span::call_site());
        stream.append_all(quote! {
            pub use flecsys::#name as #name;
        });
    }
    for s in statics {
        let name = Ident::new(&s.name, proc_macro2::Span::call_site());
        stream.append_all(quote! {
            pub use flecsys:: #name as #name;
        });
    }
    // all methods will be sent to the respective impl_stream of the opaque type
    // this will split const and mut methods into separate impls.
    let mut impl_streams = BTreeMap::new();
    for f in extern_fns {
        if should_skip_generating_fn(&f) {
            continue;
        }
        // if we reach this point, this is a method on an opaque struct
        let ExternFn {
            name,
            args,
            ret,
            doc,
            link_name,
        } = f;
        let (_, self_arg_ty) = args.first().unwrap();

        // if we reach this point, this is opaque type
        let mut fn_stream = TokenStream::new();
        let name = Ident::new(&name, proc_macro2::Span::call_site());
        // start with doc attribute
        if !doc.is_empty() {
            let doc = syn::Item::Verbatim(TokenStream::from_str(&doc).unwrap());
            fn_stream.append_all(quote! {
                #[doc = #doc]
            });
        }
        // start fn definition
        fn_stream.append_all(quote! {
            pub fn #name
        });
        // create the args first, before we append them to the stream
        let mut args_stream = TokenStream::new();
        // also prepare the stream for the args preparation by converting from rust to c types
        let mut arg_converter_stream = TokenStream::new();
        // slowly buildup the native call while we are at it.
        let mut native_call_stream = TokenStream::new();

        for (index, (arg_name, c_arg_ty)) in args.iter().enumerate() {
            let arg_name = Ident::new(arg_name, proc_macro2::Span::call_site());
            // skip the first world
            let slf = index == 0;

            c_arg_ty
                .c_arg_input_setup(
                    slf,
                    &arg_name,
                    &mut args_stream,
                    &mut arg_converter_stream,
                    &gtx,
                )
                .with_context(|| {
                    format!("error occursed while generating input for wrapper fn {link_name}")
                })?;
            native_call_stream.append_all(quote! {
                #arg_name,
            });
        }
        // append the args to the stream
        fn_stream.append_all(quote! {
        (#args_stream)
        });
        let link_name = Ident::new(&link_name, proc_macro2::Span::call_site());

        let mut return_converter_stream = TokenStream::new();
        ret.c_return_setup(&mut fn_stream, &mut return_converter_stream)
            .with_context(|| {
                format!("error occursed while generating output wrapper fn for {link_name}")
            })?;

        fn_stream.append_all(quote! {
            {
                #arg_converter_stream
                let result = unsafe {
                    flecsys::#link_name(#native_call_stream)
                };
                #return_converter_stream
            }
        });

        if !impl_streams.contains_key(self_arg_ty) {
            impl_streams.insert(self_arg_ty.clone(), TokenStream::new());
        }
        impl_streams
            .get_mut(self_arg_ty)
            .unwrap()
            .append_all(fn_stream);
    }
    for (ty, impl_stream) in impl_streams.iter_mut() {
        let rust_ty = ty.get_opaque_type().unwrap();
        let mut ty = ty.clone();
        match &mut ty {
            Types::Ptr { ty, mutable } => {
                *mutable = true;
            }
            _ => {}
        }
        // we don't want to generate struct declaration twice for const and mut methods.
        // so, we only generate for mutable.
        stream.append_all(quote! {
            pub struct #rust_ty {
                ptr: #ty,
            }
            impl #rust_ty {
                #impl_stream
            }
        });
    }
    let auto_bindigns = match syn::parse2(stream.clone()) {
        Ok(f) => prettyplease::unparse(&f),
        Err(e) => {
            eprintln!(
                "failed to format stream before writing to auto.rs
            "
            );
            stream.to_string()
        }
    };
    std::fs::write("./src/auto.rs", auto_bindigns).context("failed to write to auto.rs")?;
    Ok(())
}

fn should_skip_generating_fn(func: &ExternFn) -> bool {
    let ExternFn {
        name,
        link_name,
        args,
        ret,
        doc,
    } = func;
    if !link_name.starts_with("ecs") {
        println!("ignoring fn {name}");
        return true;
    }

    if args.iter().skip(1).map(|(_, ty)| ty).any(|arg_ty| {
        if let Types::Ptr { ty, .. } = arg_ty {
            // opaque types are fine, don't skip because of them
            if arg_ty.get_opaque_type().is_some() {
                false
            } else {
                // non opaque type pointers are a pain. so we skip them
                true
            }
        } else {
            false
        }
    }) {
        println!("ignoring fn {name} because it takes pointer arg as input");
        return true;
    }
    // skip if return type is a pointer. we can't [always] know whether its owning or borrowing
    match ret {
        Types::Ptr { .. } => {
            println!("ignoring fn {name} because it returns a pointer");
            return true;
        }
        Types::CType(c_ty) => match c_ty.as_str() {
            "ecs_ref_t" | "ecs_iter_t" => {
                println!("ignoring fn {name} because it returns a {c_ty}");
                return true;
            }
            _ => {}
        },
        _ => {}
    }
    // early continue for non-methods
    if let Some((_, self_arg)) = args.first() {
        if self_arg.get_opaque_type().is_none() {
            println!("ignoring fn {name} because first arg {self_arg}is not opaque ");
            return true;
        }
    } else {
        println!("ignoring fn {name} because it doesn't take any input");
        return true;
    };
    false
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Types {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    CType(String),
    RustType(String),
    Ptr { ty: Box<Types>, mutable: bool },
    Ref { ty: Box<Types>, mutable: bool },
    RefSelf,
    MutSelf,
}

impl ToTokens for Types {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Types::Void => tokens.append_all(quote! { () }),
            Types::Bool => tokens.append_all(quote! { bool }),
            Types::U8 => tokens.append_all(quote! { u8 }),
            Types::U16 => tokens.append_all(quote! { u16 }),
            Types::U32 => tokens.append_all(quote! { u32 }),
            Types::U64 => tokens.append_all(quote! { u64 }),
            Types::Usize => tokens.append_all(quote! { usize }),
            Types::I8 => tokens.append_all(quote! { i8 }),
            Types::I16 => tokens.append_all(quote! { i16 }),
            Types::I32 => tokens.append_all(quote! { i32 }),
            Types::I64 => tokens.append_all(quote! { i64 }),
            Types::Isize => tokens.append_all(quote! { isize }),
            Types::F32 => tokens.append_all(quote! { f32 }),
            Types::F64 => tokens.append_all(quote! { f64 }),
            Types::CType(s) => {
                let s = Ident::new(s, proc_macro2::Span::call_site());
                tokens.append_all(quote! { flecsys:: #s})
            }
            Types::Ptr { ty, mutable } => {
                let ty = ty.to_token_stream();
                if *mutable {
                    tokens.append_all(quote! { *mut #ty })
                } else {
                    tokens.append_all(quote! { *const #ty })
                }
            }
            Types::Ref { ty, mutable } => {
                let ty = ty.to_token_stream();
                if *mutable {
                    tokens.append_all(quote! { &mut #ty })
                } else {
                    tokens.append_all(quote! { & #ty })
                }
            }
            Types::RefSelf => tokens.append_all(quote! { &self }),
            Types::MutSelf => tokens.append_all(quote! { &mut self }),
            Types::RustType(rt) => {
                let rt = Ident::new(rt, proc_macro2::Span::call_site());
                tokens.append_all(quote! { #rt })
            }
        }
    }
}
impl Types {
    pub fn parse_from_type_path(s: &Type) -> Result<Self> {
        match s {
            Type::Path(p) => match p
                .path
                .segments
                .last()
                .unwrap()
                .to_token_stream()
                .to_string()
                .as_str()
            {
                "c_void" => return Ok(Types::Void),
                "bool" => return Ok(Types::Bool),
                "c_uchar" | "u8" => return Ok(Types::U8),
                "c_ushort" | "u16" => return Ok(Types::U16),
                "c_uint" | "u32" => return Ok(Types::U32),
                "c_ulong" | "u64" => return Ok(Types::U64),
                "usize" => return Ok(Types::Usize),
                "c_char" | "c_schar" | "i8" => return Ok(Types::I8),
                "c_short" | "i16" => return Ok(Types::I16),
                "c_int" | "i32" => return Ok(Types::I32),
                "i64" => return Ok(Types::I64),
                "isize" => return Ok(Types::Isize),
                "f32" => return Ok(Types::F32),
                "f64" => return Ok(Types::F64),
                rest => {
                    if rest.starts_with("ecs_") && rest.ends_with("_t") {
                        return Ok(Types::CType(rest.to_string()));
                    }
                    if rest.contains("CStr") {
                        return Ok(Types::Ref {
                            ty: Box::new(Types::RustType("std::ffi::CStr".to_string())),
                            mutable: false,
                        });
                    }
                    bail!("Unknown type path {:#?}", s)
                }
            },
            Type::Ptr(p) => {
                // check that const and mut are exclusive for sanity
                assert_eq!(p.const_token.is_some(), p.mutability.is_none());

                let ty = Types::parse_from_type_path(&*p.elem)?;
                return Ok(Types::Ptr {
                    ty: Box::new(ty),
                    mutable: p.mutability.is_some(),
                });
            }
            Type::Reference(r) => {
                let ty = Types::parse_from_type_path(&*r.elem)?;
                return Ok(Types::Ref {
                    ty: Box::new(ty),
                    mutable: r.mutability.is_some(),
                });
            }
            _ => bail!("Unknown type {:#?}", s),
        }
    }

    /// The c function's return type will be "mapped" to the specific rust types
    /// If a c function `ecs_foo` returns `ecs_entity_t`, you can map `ecs_entity_t` to `u64` or `Entity(u64)` here
    ///
    /// # Arguments
    /// - self is the c type that the fn returns.
    /// - return_ty is the stream where we will add `-> rust_ty` to the fn signature (unless there's no return type like void).
    /// - return_ty_converter is the stream where we will convert the c return type to the rust return type. eg: `let result: rust_ty = result.try_into().unwrap();`
    ///
    /// We assume that the native fn call's output will be stored in a variable called `result`. eg: `let result = ecs_foo()`
    fn c_return_setup(
        &self,
        return_ty_specifier_stream: &mut TokenStream,
        return_ty_converter: &mut TokenStream,
    ) -> Result<()> {
        match self {
            Types::Void => {}
            Types::Bool
            | Types::U8
            | Types::U16
            | Types::U32
            | Types::U64
            | Types::Usize
            | Types::I8
            | Types::I16
            | Types::I32
            | Types::I64
            | Types::Isize
            | Types::F32
            | Types::F64 => {
                return_ty_specifier_stream.append_all(quote! {
                    -> #self
                });
                return_ty_converter.append_all(quote! {
                    return result;
                });
            }
            Types::CType(c_ty) => {
                match c_ty.as_str() {
                    // type aliases
                    "ecs_id_t" | "ecs_entity_t" => {
                        return_ty_specifier_stream.append_all(quote! {
                            -> u64
                        });
                        return_ty_converter.append_all(quote! {
                            return result;
                        });
                    }
                    "ecs_flags32_t" => {
                        return_ty_specifier_stream.append_all(quote! {
                            -> u32
                        });
                        return_ty_converter.append_all(quote! {
                            return result;
                        });
                    }
                    "ecs_flags8_t" => {
                        return_ty_specifier_stream.append_all(quote! {
                            -> u8
                        });
                        return_ty_converter.append_all(quote! {
                            return result;
                        });
                    }
                    "ecs_flags16_t" => {
                        return_ty_specifier_stream.append_all(quote! {
                            -> u16
                        });
                        return_ty_converter.append_all(quote! {
                            return result;
                        });
                    }

                    _ => bail!("unknown c type {c_ty} in return type"),
                }
            }
            _ => bail!("unknown return type {self}"),
        }
        Ok(())
    }
    /// Rust wrappers for c functions will need to take rust types and map them to c types
    /// This function will do two things
    /// 1. Prepare the args_stream for the function signature by specifying the rust types. eg: `arg_name: rust_ty`
    /// 2. Prepare the args_converter_stream for the function body by converting the rust types to c types. eg: `let arg_name: c_ty = arg_name.try_into().unwrap();`
    ///
    /// After calling this fn, you can just call the native fn. eg: `unsafe { flecsys::ecs_foo(#args_stream) }`
    ///
    /// # Arguments
    /// * `self` - the C type. This is the type that the c function expects.
    /// * `first_arg` - if this is the first argument in the function signature. This is useful for methods on opaque types, as we will use `&self` / `&mut self`
    /// * `arg_name` - the name of the argument
    /// * `args_stream` - the fn signature stream to append the paramter list of the inputs
    /// * `args_converter_stream` - the fn body stream where we convert the rust types to c types
    fn c_arg_input_setup(
        &self,
        first_arg: bool,
        arg_name: &Ident,
        args_stream: &mut TokenStream,
        args_converter_stream: &mut TokenStream,
        gtx: &GeneratorContext,
    ) -> Result<()> {
        match self {
            Types::Void => {
                bail!("void type as input parameter");
            }
            Types::Bool
            | Types::U8
            | Types::U16
            | Types::U32
            | Types::U64
            | Types::I8
            | Types::I16
            | Types::I32
            | Types::I64
            | Types::F32
            | Types::F64 => {
                args_stream.append_all(quote! {
                    #arg_name: #self,
                });
                return Ok(());
            }
            Types::Usize => {
                args_stream.append_all(quote! {
                    #arg_name: #self,
                });
                return Ok(());
            }
            Types::Isize => {
                args_stream.append_all(quote! {
                    #arg_name: #self,
                });
                return Ok(());
            }
            Types::CType(c_ty) => {
                match c_ty.as_str() {
                    // type aliases
                    "ecs_id_t" | "ecs_entity_t" => {
                        args_stream.append_all(quote! {
                            #arg_name: u64,
                        });
                        return Ok(());
                    }
                    "ecs_flags32_t" => {
                        args_stream.append_all(quote! {
                            #arg_name: u32,
                        });
                    }
                    "ecs_flags8_t" => {
                        args_stream.append_all(quote! {
                            #arg_name: u8,
                        });
                    }
                    "ecs_flags16_t" => {
                        args_stream.append_all(quote! {
                            #arg_name: u16,
                        });
                    }
                    rest => {
                        // raw C functions
                        if rest.ends_with("action_t") {
                            args_stream.append_all(quote! {
                                #arg_name: #self,
                            });
                        } else {
                            bail!("unknown c type {c_ty} in input parameters");
                        }
                    }
                }
            }
            Types::RustType(r) => {
                args_stream.append_all(quote! {
                    #arg_name: #r,
                });
            }
            Types::Ptr { ty, mutable } => {
                let mutable = *mutable;
                match ty.as_ref() {
                    Types::CType(_) => {
                        let rust_ty = self.get_opaque_type().ok_or_else(|| {
                            anyhow::anyhow!("non opaque type as pointer parameter, got {self}")
                        })?;
                        if first_arg {
                            args_stream.append_all(if mutable {
                                quote! {
                                    &mut self,
                                }
                            } else {
                                quote! {
                                    &self,
                                }
                            });
                            args_converter_stream.append_all(if mutable {
                                quote! {
                                    let #arg_name = self.ptr;
                                }
                            } else {
                                quote! {
                                    let #arg_name = self.ptr.cast_const();
                                }
                            });
                        } else {
                            args_stream.append_all(if mutable {
                                quote! {
                                    #arg_name: &mut #rust_ty,
                                }
                            } else {
                                quote! {
                                    #arg_name: & #rust_ty,
                                }
                            });
                            args_converter_stream.append_all(if mutable {
                                quote! {
                                    let #arg_name = #arg_name.ptr;
                                }
                            } else {
                                quote! {
                                    let #arg_name = #arg_name.ptr.cast_const();
                                }
                            });
                        }
                    }
                    _ => bail!("unknown ptr type {ty} in input parameters"),
                }
            }
            Types::Ref { .. } => {
                bail!("Ref as c input parameter");
            }
            Types::RefSelf | Types::MutSelf => {
                bail!("Ref/MutSelf as c input parameter");
            }
        }
        Ok(())
    }
    /// checks if self is an opaque type.
    fn get_opaque_type(&self) -> Option<Ident> {
        let is_mutable;
        match self {
            Types::Ptr { ty, mutable } => {
                is_mutable = *mutable;
                match ty.as_ref() {
                    Types::CType(c_ty) => match c_ty.as_str() {
                        "ecs_world_t"
                        | "ecs_stage_t"
                        | "ecs_table_t"
                        | "ecs_id_record_t"
                        | "ecs_table_record_t"
                        | "ecs_mixins_t"
                        | "ecs_data_t"
                        | "ecs_query_cache_table_match_t"
                        | "ecs_http_server_t"
                        | "ecs_script_template_t" => {
                            let rust_ty = c_ty
                                .strip_prefix("ecs_")
                                .unwrap()
                                .strip_suffix("_t")
                                .unwrap();
                            return Some(Ident::new(
                                &format!(
                                    "{}{}",
                                    heck::AsPascalCase(rust_ty),
                                    if is_mutable { "Mut" } else { "Ref" }
                                ),
                                proc_macro2::Span::call_site(),
                            ));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        };
        None
    }
}
impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Void => write!(f, "void"),
            Types::Bool => write!(f, "bool"),
            Types::U8 => write!(f, "u8"),
            Types::U16 => write!(f, "u16"),
            Types::U32 => write!(f, "u32"),
            Types::U64 => write!(f, "u64"),
            Types::Usize => write!(f, "usize"),
            Types::I8 => write!(f, "i8"),
            Types::I16 => write!(f, "i16"),
            Types::I32 => write!(f, "i32"),
            Types::I64 => write!(f, "i64"),
            Types::Isize => write!(f, "isize"),
            Types::F32 => write!(f, "f32"),
            Types::F64 => write!(f, "f64"),
            Types::CType(s) => write!(f, "flecsys::{}", s),
            Types::Ptr { ty, mutable } => {
                write!(f, "*{}{}", if *mutable { "mut " } else { "" }, ty)
            }
            Types::Ref { ty, mutable } => {
                write!(f, "&{}{}", if *mutable { "mut " } else { "" }, ty)
            }
            Types::RefSelf => write!(f, "&self"),
            Types::MutSelf => write!(f, "&mut self"),
            Types::RustType(rt) => write!(f, "{}", rt),
        }
    }
}
pub struct ExternStatic {
    pub name: String,
    pub ty: Types,
    pub doc: String,
    pub mutable: bool,
}

impl ExternStatic {
    pub fn new(name: String, ty: Types, doc: String, mutable: bool) -> Self {
        Self {
            name,
            ty,
            doc,
            mutable,
        }
    }
    pub fn parse_from(constant: syn::ForeignItemStatic) -> Result<Self> {
        let name = constant.ident.to_string();
        let ty = Types::parse_from_type_path(&constant.ty)
            .with_context(|| format!("failed to parse type of {name}"))?;
        let doc = constant.attrs.iter().find_map(|attr| {
            if let syn::Meta::NameValue(name_value) = &attr.meta {
                if name_value.path.is_ident("doc") {
                    if let Expr::Lit(lit) = &name_value.value {
                        return Some(lit.lit.to_token_stream().to_string());
                    }
                }
            }
            None
        });
        Ok(Self::new(
            name,
            ty,
            doc.unwrap_or_default(),
            constant.mutability != StaticMutability::None,
        ))
    }
}
pub struct ExternFn {
    pub name: String,
    pub link_name: String,
    pub args: Vec<(String, Types)>,
    pub ret: Types,
    pub doc: String,
}
impl ExternFn {
    pub fn new(
        name: String,
        link_name: String,
        args: Vec<(String, Types)>,
        ret: Types,
        doc: String,
    ) -> Self {
        Self {
            name,
            args,
            ret,
            doc,
            link_name,
        }
    }
    pub fn parse_from(
        gtx: &mut GeneratorContext,
        foreign_fn: ForeignItemFn,
    ) -> anyhow::Result<Self> {
        let link_name = foreign_fn.sig.ident.to_string();
        let name = gtx
            .explicit_rename_fns
            .get(&link_name)
            .cloned()
            .unwrap_or_else(|| {
                link_name
                    .strip_prefix("ecs_")
                    .unwrap_or(&link_name)
                    .to_string()
            });
        let mut args = Vec::new();
        for arg in foreign_fn.sig.inputs.iter() {
            if let syn::FnArg::Typed(typed) = arg {
                let arg_name = if let syn::Pat::Ident(ident) = &*typed.pat {
                    ident.ident.to_string()
                } else {
                    bail!("failed to parse paramter name {:#?}", typed.pat);
                };
                match Types::parse_from_type_path(&typed.ty) {
                    Ok(ty) => {
                        args.push((arg_name, ty));
                    }
                    Err(e) => {
                        bail!(
                            "failed to parater parameter {arg_name}'s type {:#?}: {e}",
                            typed.ty
                        );
                    }
                }
            }
        }
        let ret = if let syn::ReturnType::Type(_, ty) = &foreign_fn.sig.output {
            Types::parse_from_type_path(&ty).with_context(|| name.clone())?
        } else {
            Types::Void
        };

        let doc = foreign_fn.attrs.iter().find_map(|attr| {
            if let syn::Meta::NameValue(name_value) = &attr.meta {
                if name_value.path.is_ident("doc") {
                    if let Expr::Lit(lit) = &name_value.value {
                        return Some(lit.lit.to_token_stream().to_string());
                    }
                }
            }
            None
        });
        Ok(Self::new(
            name,
            link_name,
            args,
            ret,
            doc.unwrap_or_default(),
        ))
    }
}
pub struct Constant {
    pub name: String,
    pub ty: Types,
    pub doc: String,
}
impl Constant {
    pub fn new(name: String, ty: Types, doc: String) -> Self {
        Self { name, ty, doc }
    }
    pub fn parse_from(constant: syn::ItemConst) -> Result<Self> {
        let name = constant.ident.to_string();
        let ty = Types::parse_from_type_path(&constant.ty)?;
        let doc = constant.attrs.iter().find_map(|attr| {
            if let syn::Meta::NameValue(name_value) = &attr.meta {
                if name_value.path.is_ident("doc") {
                    if let Expr::Lit(lit) = &name_value.value {
                        return Some(lit.lit.to_token_stream().to_string());
                    }
                }
            }
            None
        });
        Ok(Self::new(name, ty, doc.unwrap_or_default()))
    }
}
fn any_args_usize<'a>(args: impl IntoIterator<Item = &'a syn::FnArg>) -> bool {
    for arg in args.into_iter() {
        if let syn::FnArg::Typed(typed) = arg {
            if let syn::Type::Path(path) = &*typed.ty {
                if path.path.segments.last().unwrap().ident == "usize" {
                    return true;
                }
            }
        }
    }
    false
}
struct BindingsTransformer;
impl Fold for BindingsTransformer {
    fn fold_foreign_item_fn(&mut self, mut i: syn::ForeignItemFn) -> syn::ForeignItemFn {
        let name = i.sig.ident.to_string();
        if name.starts_with("ecs")
            && (any_args_usize(&i.sig.inputs) || any_output_usize(&i.sig.output))
        {
            set_fn_usize_to_u64(&mut i);
        }
        i
    }
    fn fold_type_bare_fn(&mut self, mut i: syn::TypeBareFn) -> syn::TypeBareFn {
        if any_bare_args_usize(&i.inputs) || any_output_usize(&i.output) {
            set_bare_fn_usize_to_u64(&mut i);
        }
        i
    }
    fn fold_item_struct(&mut self, i: syn::ItemStruct) -> syn::ItemStruct {
        i
    }
}
fn any_bare_args_usize<'a>(args: impl IntoIterator<Item = &'a syn::BareFnArg>) -> bool {
    for arg in args.into_iter() {
        if let syn::Type::Path(path) = &arg.ty {
            if path.path.segments.last().unwrap().ident == "usize" {
                return true;
            }
        }
    }
    false
}
fn any_output_usize(output: &syn::ReturnType) -> bool {
    match output {
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            syn::Type::Path(t) => {
                if t.path.segments.last().unwrap().ident == "usize" {
                    return true;
                }
            }
            _ => {}
        },
        _ => {}
    }
    false
}
fn replace_usize_with_u64(ty: &mut syn::Type) {
    match ty {
        syn::Type::Path(path) => {
            if path.path.segments[0].ident == "usize" {
                path.path.segments[0].ident = Ident::new("u64", path.path.segments[0].ident.span())
            }
        }
        _ => {}
    }
}
fn set_fn_usize_to_u64(i: &mut ForeignItemFn) {
    i.sig.inputs = i
        .sig
        .inputs
        .iter()
        .cloned()
        .map(|mut input| {
            if let syn::FnArg::Typed(typed) = &mut input {
                replace_usize_with_u64(&mut typed.ty);
            }
            input
        })
        .collect();
    match &mut i.sig.output {
        syn::ReturnType::Default => {}
        syn::ReturnType::Type(_, ty) => replace_usize_with_u64(ty),
    }
}

fn set_bare_fn_usize_to_u64(i: &mut TypeBareFn) {
    i.inputs = i
        .inputs
        .iter()
        .cloned()
        .map(|mut input| {
            replace_usize_with_u64(&mut input.ty);
            input
        })
        .collect();
    match &mut i.output {
        syn::ReturnType::Default => {}
        syn::ReturnType::Type(_, ty) => replace_usize_with_u64(ty),
    }
}
