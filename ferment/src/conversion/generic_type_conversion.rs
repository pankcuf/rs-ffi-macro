use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, Generics, ParenthesizedGenericArguments, parse_quote, Pat, PathArguments, PathSegment, PatType, ReturnType, Type, TypeParamBound, TypePath, TypeSlice, Visibility, VisPublic};
use syn::__private::TokenStream2;
use syn::token::Brace;
use crate::ast::{AddPunctuated, BraceWrapped, CommaPunctuated, Depunctuated, ParenWrapped};
use crate::composable::{FieldComposer, FieldTypeConversionKind};
use crate::composer::{ComposerPresenter, struct_composer_ctor_root, ParentComposer, STRUCT_COMPOSER_CTOR_NAMED_ITEM, FromConversionComposer, Composer};
use crate::context::ScopeContext;
use crate::conversion::{expand_attributes, TypeConversion};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg, Mangle, Resolve, SpecialType, Terminated, ToPath, ToType};
use crate::presentable::{BindingPresentableContext, ConstructorBindingPresentableContext, ConstructorPresentableContext, Expression, OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, create_callback, create_struct, DictionaryExpr, DictionaryName, DropInterfacePresentation, FFIConversionMethod, FFIConversionMethodExpr, FFIObjectPresentation, FFIVecConversionMethodExpr, InterfacePresentation, InterfacesMethodExpr, Name};

pub type InterfacesMethodComposer = ComposerPresenter<TokenStream2, InterfacesMethodExpr>;
pub const FROM_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptPrimitive(expr);
pub const TO_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptPrimitive(expr);
pub const FROM_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFrom, expr);
pub const FROM_OPT_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFromOpt, expr);
pub const TO_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiTo, expr);
pub const TO_OPT_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiToOpt, expr);

pub const FROM_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromPrimitiveGroup(expr);
pub const FROM_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromComplexGroup(expr);
pub const FROM_OPT_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptPrimitiveGroup(expr);
pub const FROM_OPT_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptComplexGroup(expr);

pub const TO_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToPrimitiveGroup(expr);
pub const TO_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToComplexGroup(expr);
pub const TO_OPT_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptPrimitiveGroup(expr);
pub const TO_OPT_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptComplexGroup(expr);

pub const DESTROY_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxVecPtr(expr);
pub const DESTROY_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxAnyVecPtr(expr);
pub const DESTROY_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxAny(expr);
pub const DESTROY_OPT_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxAnyOpt(expr);
// Expression::DestroyOpt(expr.into())
pub const DESTROY_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::DestroyOptPrimitive(expr);

pub struct GenericArgComposer {
    // pub ty: Type,
    pub from_composer: InterfacesMethodComposer,
    pub to_composer: InterfacesMethodComposer,
    pub destroy_composer: InterfacesMethodComposer,
}
impl GenericArgComposer {
    pub fn new(from_composer: InterfacesMethodComposer, to_composer: InterfacesMethodComposer, destroy_composer: InterfacesMethodComposer) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }
    pub fn from(&self, expr: TokenStream2) -> Expression {
        Expression::InterfacesExpr((self.from_composer)(expr))

    }
    pub fn to(&self, expr: TokenStream2) -> Expression {
        Expression::InterfacesExpr((self.to_composer)(expr))

    }
    pub fn destroy(&self, expr: TokenStream2) -> Expression {
        Expression::InterfacesExpr((self.destroy_composer)(expr))
    }
}

pub type GenericNestedArgComposer = fn(arg_name: &Name, arg_ty: &Type) -> GenericArgPresentation;

#[allow(unused)]
pub trait GenericNamedArgComposer {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation;
}

impl GenericNamedArgComposer for Type {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation {
        composer(name, self)
    }
}

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: Expression,
    pub from_conversion: Expression,
    pub to_conversion: Expression,
}

impl Debug for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl Display for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: Expression, from_conversion: Expression, to_conversion: Expression) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum GenericTypeConversion {
    Map(Type),
    IndexMap(Type),
    SerdeJsonMap(Type),
    Vec(Type),
    BTreeSet(Type),
    HashSet(Type),
    Result(Type),
    Box(Type),
    AnyOther(Type),
    Array(Type),
    Slice(Type),
    Tuple(Type),
    Optional(Type),
    Callback(Type),
    TraitBounds(AddPunctuated<TypeParamBound>),
}
impl Debug for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericTypeConversion::{}({})", match self {
            GenericTypeConversion::Map(_) => "Map",
            GenericTypeConversion::IndexMap(_) => "IndexMap",
            GenericTypeConversion::SerdeJsonMap(_) => "SerdeJsonMap",
            GenericTypeConversion::Vec(_) => "Vec",
            GenericTypeConversion::BTreeSet(_) => "BTreeSet",
            GenericTypeConversion::HashSet(_) => "HashSet",
            GenericTypeConversion::Result(_) => "Result",
            GenericTypeConversion::Box(_) => "Box",
            GenericTypeConversion::AnyOther(_) => "AnyOther",
            GenericTypeConversion::Array(_) => "Array",
            GenericTypeConversion::Slice(_) => "Slice",
            GenericTypeConversion::Tuple(_) => "Tuple",
            GenericTypeConversion::Callback(_) => "Callback",
            GenericTypeConversion::TraitBounds(_) => "TraitBounds",
            GenericTypeConversion::Optional(_) => "Optional"
        }, self.to_token_stream()))
    }
}
impl Display for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for GenericTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Optional(ty) |
            GenericTypeConversion::Callback(ty) |
            GenericTypeConversion::Tuple(ty) => ty.to_tokens(tokens),
            GenericTypeConversion::TraitBounds(bounds) => bounds.to_tokens(tokens),
        }
    }
}
impl GenericTypeConversion {
    pub fn ty(&self) -> Option<&Type> {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Callback(ty) |
            GenericTypeConversion::Tuple(ty) => Some(ty),
            GenericTypeConversion::Optional(Type::Path(TypePath { qself: _, path })) => match path.segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => Some(ty),
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
                },
                _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
            }
            GenericTypeConversion::TraitBounds(_) => {
                // TODO: Make mixin here
                None
            },
            conversion => panic!("TODO: Non-supported generic conversion: {}", conversion),
        }
    }
}

impl GenericTypeConversion {

    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        let attrs = expand_attributes(attrs);
        println!("Generic::Expand: {} ---- {}", self, quote!(#(#attrs)*));

        match self {
            GenericTypeConversion::Result(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryName::Error);
                let compose_arg = |arg_ty: Type, from_expr: Expression, to_expr: Expression, destroy_expr: Expression|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        Expression::MapExpression(Expression::O.into(), from_expr.into()),
                        to_expr);


                let compose = |arg_name: &Name, ty: &Type| {
                    println!("RESULT ARG: {} -- {}", arg_name, ty.to_token_stream());
                    // let from_composer = FromConversionComposer::new(arg_name.clone(), ty.clone(), Some(Expression::DictionaryName(DictionaryName::O)));
                    // let from = from_composer.compose(&source);
                    match TypeConversion::from(ty) {
                        TypeConversion::Primitive(arg_ty) => {
                            compose_arg(
                                arg_ty.clone(),
                                // from,
                                Expression::Deref(DictionaryName::O.to_token_stream()),
                                Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(DictionaryName::O.to_token_stream())),
                                Expression::InterfacesExpr(DESTROY_OPT_PRIMITIVE(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream())))
                        }
                        TypeConversion::Complex(arg_ty) => {

                            let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX);
                            compose_arg(
                                arg_ty.special_or_to_ffi_full_path_type(&source),
                                // from,
                                arg_composer.from(DictionaryName::O.to_token_stream()),
                                arg_composer.to(DictionaryName::O.to_token_stream()),
                                arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                        }
                        TypeConversion::Generic(generic_arg_ty) => {
                            let (arg_composer, arg_ty) = match generic_arg_ty {
                                GenericTypeConversion::Optional(_) => match generic_arg_ty.ty() {
                                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                    Some(ty) => match TypeConversion::from(ty) {
                                        TypeConversion::Primitive(_) => (GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.special_or_to_ffi_full_path_type(&source)),
                                        TypeConversion::Generic(nested_nested) => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), nested_nested.special_or_to_ffi_full_path_type(&source)),
                                        _ => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), ty.special_or_to_ffi_full_path_type(&source)),
                                    }
                                },
                                GenericTypeConversion::Box(_) => match generic_arg_ty.ty().and_then(|ty| ty.first_nested_type()) {
                                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                    Some(ty) => match TypeConversion::from(ty) {
                                        TypeConversion::Primitive(_) => (GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.special_or_to_ffi_full_path_type(&source)),
                                        TypeConversion::Generic(nested_nested) => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), nested_nested.special_or_to_ffi_full_path_type(&source)),
                                        _ => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), ty.special_or_to_ffi_full_path_type(&source)),
                                    }
                                },
                                _ => (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(&source)),
                            };

                            let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                                match generic_arg_ty.ty() {
                                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                    Some(ty) => match TypeConversion::from(ty) {
                                        TypeConversion::Primitive(_) => (GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.special_or_to_ffi_full_path_type(&source)),
                                        TypeConversion::Generic(nested_nested) => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), nested_nested.special_or_to_ffi_full_path_type(&source)),
                                        _ => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), ty.special_or_to_ffi_full_path_type(&source)),
                                    }
                                }


                            } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(&source)) };
                            compose_arg(
                                arg_ty,
                                // from,
                                arg_composer.from(DictionaryName::O.to_token_stream()),
                                arg_composer.to(DictionaryName::O.to_token_stream()),
                                arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                        }
                    }
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                let field_names = CommaPunctuated::from_iter([arg_0_name.clone(), arg_1_name.clone()]);
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Type(arg_0_presentation.ty.joined_mut())),
                        FieldComposer::named(arg_1_name.clone(), FieldTypeConversionKind::Type(arg_1_presentation.ty.joined_mut())),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                DictionaryExpr::FromRoot(
                                    InterfacesMethodExpr::FoldToResult(CommaPunctuated::from_iter([
                                        Expression::FfiRefWithName(arg_0_name),
                                        Expression::FfiRefWithName(arg_1_name),
                                        arg_0_presentation.from_conversion,
                                        arg_1_presentation.from_conversion,
                                    ]).present(&source).to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                DictionaryExpr::Depunctuated(
                                    Depunctuated::from_iter([
                                        // Stmt::Local(Local {
                                        //     attrs: vec![],
                                        //     let_token: Default::default(),
                                        //     pat: Pat::Type(PatType {
                                        //         attrs: vec![],
                                        //         pat: Box::new(Pat::Ident(PatIdent {
                                        //             attrs: vec![],
                                        //             by_ref: None,
                                        //             mutability: None,
                                        //             ident: Ident::new(),
                                        //             subpat: None,
                                        //         })),
                                        //         colon_token: Default::default(),
                                        //         ty: Box::new(Type::__NonExhaustive),
                                        //     }),
                                        //     init: Some((syn::token::Eq(), Box::new(Expr::))),
                                        //     semi_token: Default::default(),
                                        // }).to_token_stream(),
                                        //
                                        // Stmt::Expr(Expr::Struct(ExprStruct {
                                        //     attrs: vec![],
                                        //     path: parse_quote!(Self),
                                        //     brace_token: Default::default(),
                                        //     fields: Punctuated::from_iter([
                                        //         FieldValue { attrs: vec![], member: Member::Named(arg_0_name.mangle_ident_default()), colon_token: None, expr: Expr::__NonExhaustive, },
                                        //         FieldValue { attrs: vec![], member: Member::Named(arg_1_name.mangle_ident_default()), colon_token: None, expr: Expr::__NonExhaustive, },
                                        //     ]),
                                        //     dot2_token: None,
                                        //     rest: None,
                                        // })).to_token_stream(),
                                        //
                                        // Expr::Call(ExprCall  {
                                        //     attrs: vec![],
                                        //     func: Box::new(Expr::Path(ExprPath {
                                        //         attrs: vec![],
                                        //         qself: None,
                                        //         path: Path { leading_colon: None, segments: Punctuated::from_iter([]) },
                                        //     })),
                                        //     paren_token: Default::default(),
                                        //     args: Default::default(),
                                        // })

                                        DictionaryExpr::LetExpr(
                                            quote!((#field_names)),
                                            InterfacesMethodExpr::ToResult(
                                                CommaPunctuated::from_iter([
                                                    DictionaryName::Obj.to_token_stream(),
                                                    DictionaryExpr::Mapper(
                                                        DictionaryName::O.to_token_stream(),
                                                        arg_0_presentation.to_conversion.present(&source)).to_token_stream(),
                                                    DictionaryExpr::Mapper(
                                                        DictionaryName::O.to_token_stream(),
                                                        arg_1_presentation.to_conversion.present(&source)).to_token_stream(),
                                                ]).to_token_stream())
                                                .to_token_stream()
                                                .terminated())
                                            .to_token_stream(),
                                        InterfacesMethodExpr::Boxed(DictionaryExpr::SelfDestructuring(field_names.to_token_stream()).to_token_stream()).to_token_stream(),


                                    ])).to_token_stream(),
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([arg_0_presentation.destructor.present(&source).terminated(), arg_1_presentation.destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty)=> {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);

                let arg_context = |arg_name: &Name| quote!(obj.#arg_name().cloned());
                let arg_args = |arg_name: &Name| CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);

                let compose_arg = |arg_ty: Type, from_expr: Expression, to_expr: Expression, destroy_expr: Expression|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        Expression::MapExpression(Expression::O.into(), from_expr.into()),
                        to_expr);
                let compose = |arg_name: &Name, ty: &Type| match TypeConversion::from(ty) {
                    TypeConversion::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            Expression::O,
                            Expression::InterfacesExpr(TO_PRIMITIVE_GROUP(arg_context(arg_name))),
                            Expression::InterfacesExpr(DESTROY_PRIMITIVE_GROUP(arg_args(arg_name).to_token_stream())))
                    },
                    TypeConversion::Complex(arg_ty) => {
                        let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP);
                        compose_arg(
                            arg_ty.special_or_to_ffi_full_path_variable_type(&source),
                            arg_composer.from(DictionaryName::O.to_token_stream()).into(),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                    TypeConversion::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => (match TypeConversion::from(ty) {
                                    TypeConversion::Primitive(_) => GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP),
                                    _ => GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP),
                                }, ty.special_or_to_ffi_full_path_variable_type(&source))
                            }
                        } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), generic_arg_ty.special_or_to_ffi_full_path_variable_type(&source)) };
                        compose_arg(
                            arg_ty,
                            arg_composer.from(DictionaryName::O.to_token_stream()),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
                        FieldComposer::named(arg_0_name, FieldTypeConversionKind::Type(key.joined_mut())),
                        FieldComposer::named(arg_1_name, FieldTypeConversionKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                {
                                    let ffi_ref = DictionaryName::FfiRef;
                                    let count = DictionaryName::Count;
                                    let keys = DictionaryName::Keys;
                                    let values = DictionaryName::Values;
                                    let args = CommaPunctuated::from_iter([
                                        quote!(#ffi_ref.#count),
                                        quote!(#ffi_ref.#keys),
                                        quote!(#ffi_ref.#values),
                                        from_key_conversion.present(&source),
                                        from_value_conversion.present(&source),
                                    ]);
                                    DictionaryExpr::FromRoot(InterfacesMethodExpr::FoldToMap(args.to_token_stream()).to_token_stream())
                                        .to_token_stream()
                                },
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter([
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Count), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Keys), FieldTypeConversionKind::Conversion(to_key_conversion.present(&source))),
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Values), FieldTypeConversionKind::Conversion(to_value_conversion.present(&source))),
                                        ]).to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor.present(&source).terminated(), value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Vec(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    ty.clone(),
                    TypeConversion::from(nested_ty),
                    FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream(),
                    FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream(),
                    attrs,
                    &source)
            },
            GenericTypeConversion::Array(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    parse_quote!(Vec<#nested_ty>),
                    TypeConversion::from(nested_ty),
                    DictionaryExpr::TryIntoUnwrap(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()).to_token_stream(),
                    FFIVecConversionMethodExpr::Encode(DictionaryExpr::ObjToVec.to_token_stream()).to_token_stream(),
                    attrs,
                    &source)
            },
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_slice: TypeSlice = parse_quote!(#ty);
                let elem_type = &type_slice.elem;
                let target_type: Type = parse_quote!(Vec<#elem_type>);
                let arg_0_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);
                let self_props = CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);
                let arg_0_destroy = |composer: InterfacesMethodComposer|
                    Expression::InterfacesExpr(composer(self_props.to_token_stream()));
                let arg_0_from = |composer: InterfacesMethodComposer|
                    Expression::InterfacesExpr(composer(self_props.to_token_stream()));
                let arg_0_to = |composer: InterfacesMethodComposer|
                    Expression::InterfacesExpr(
                        InterfacesMethodExpr::Boxed(
                            DictionaryExpr::SelfDestructuring(
                                CommaPunctuated::from_iter([
                                    FieldComposer::named(count_name.clone(), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                                    FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).to_token_stream()))]).to_token_stream())
                                .to_token_stream()));

                let arg_0_presentation = match TypeConversion::from(&type_slice.elem) {
                    TypeConversion::Primitive(arg_0_target_path) => {
                        GenericArgPresentation::new(
                            arg_0_target_path.clone(),
                            arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                            arg_0_from(FROM_PRIMITIVE_GROUP),
                            arg_0_to(TO_PRIMITIVE_GROUP))
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        GenericArgPresentation::new(
                            arg_0_target_ty.special_or_to_ffi_full_path_variable_type(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        GenericArgPresentation::new(
                            arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
                        FieldComposer::named(arg_0_name, FieldTypeConversionKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream(),
                                FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream(),
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs, types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::Tuple(Type::Tuple(type_tuple)) => {
                let ffi_name = type_tuple.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let tuple_items = type_tuple.elems.iter()
                    .enumerate()
                    .map(|(index, ty)|
                        dictionary_generic_arg_pair(
                            Name::UnnamedArg(index),
                            Name::Index(index),
                            ty,
                            &source))
                    .collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, _))|
                                FieldComposer::unnamed(Name::UnnamedArg(index), FieldTypeConversionKind::Type(root_path.clone())))),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_as_type, parse_quote!(#type_tuple)),
                            conversions: (
                                DictionaryExpr::FromRoot(
                                    ParenWrapped::new(
                                        CommaPunctuated::from_iter(
                                            tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.clone()))))
                                        .present(&source))
                                    .to_token_stream(),
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter(
                                            tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.clone())))
                                            .present(&source)
                                            .to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source).terminated()))),
                    &source
                )
            },
            GenericTypeConversion::AnyOther(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Obj);

                let path = ty.to_path();
                let mut constructor = path.clone();
                constructor.segments.last_mut().unwrap().arguments = PathArguments::None;

                // Arc/Rc: primitive arg: to: "*obj"
                // Arc/Rc: complex arg: to: "(*obj).clone()"
                // Mutex/RwLock: primitive/complex arg: to: "obj.into_inner().expect("Err")"
                // Arc<RwLock>>: to: obj.borrow().clone()
                // RefCell: primitive/complex arg: to: "obj.into_inner()"

                let arg_to_conversion = match &path.segments.last() {
                    Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                        "Arc" | "Rc" => {
                            let nested_ty = ty.first_nested_type().unwrap();

                            match TypeConversion::from(nested_ty) {
                                TypeConversion::Primitive(_) => DictionaryExpr::Deref(arg_0_name.to_token_stream()).to_token_stream(),
                                TypeConversion::Complex(_) => quote!((*#arg_0_name).clone()),
                                TypeConversion::Generic(nested_generic_ty) => {
                                    println!("GENERIC inside Arc/Rc: {}", nested_generic_ty);
                                    match nested_generic_ty {
                                        GenericTypeConversion::AnyOther(ty) => {
                                            println!("GENERIC (AnyOther) inside Arc/Rc: {}", ty.to_token_stream());
                                            let path = ty.to_path();
                                            match &path.segments.last() {
                                                Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                                                    "RwLock" | "Mutex" => quote!(std::sync::#ident::new(obj.read().expect("Poisoned").clone())),
                                                    _ => quote!((*#arg_0_name).clone())
                                                },
                                                None => {
                                                    panic!("Error Generic Expansion (AnyOther): {}", ty.to_token_stream())
                                                }
                                            }
                                        },
                                        _ => quote!((*#arg_0_name).clone())
                                    }
                                },
                            }
                        },
                        "Mutex" | "RwLock" => quote!(#arg_0_name.into_inner().expect("Err")),
                        // "Mutex" | "RwLock" => quote!(#arg_0_name.borrow().clone()),
                        "RefCell" => quote!(#arg_0_name.into_inner()),
                        "Pin" => quote!(&**#arg_0_name),
                        _ => { return quote! {}; }
                    }
                    None => {
                        panic!("Error Generic Expansion (AnyOther): {}", ty.to_token_stream())
                    }
                };

                let compose_arg = |arg_ty: Type, from_expr: Expression, to_expr: Expression, destroy_expr: Expression|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        from_expr,
                        to_expr);
                let compose = |arg_name: &Name, ty: &Type| match TypeConversion::from(ty) {
                    TypeConversion::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            Expression::FfiRefWithName(Name::Dictionary(DictionaryName::Obj)),
                            Expression::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter([
                                            FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_to_conversion.to_token_stream()))
                                        ]).to_token_stream())
                                        .to_token_stream())),
                            Expression::Empty)
                    }
                    TypeConversion::Complex(arg_ty) => {
                        let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX);
                        compose_arg(
                            arg_ty.special_or_to_ffi_full_path_type(&source),
                            arg_composer.from(quote!(ffi_ref.#arg_0_name)),
                            Expression::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter([
                                            FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_composer.to(arg_to_conversion).present(&source)))
                                        ]).to_token_stream()).to_token_stream())),
                            arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                    }
                    TypeConversion::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                            println!("GEEEEEE.1: {}", generic_arg_ty);
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => {
                                    println!("GEEEEEE.2: {}", ty.to_token_stream());

                                    (match TypeConversion::from(ty) {
                                        TypeConversion::Primitive(_) => GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE),
                                        TypeConversion::Generic(GenericTypeConversion::Box(..)) => {
                                            //                                    Expression::IntoBox(Expression::From(field_path.into()).into())
                                            GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_COMPLEX)
                                        }
                                        _ => GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_COMPLEX),
                                    }, ty.special_or_to_ffi_full_path_type(&source))
                                }
                            }

                        } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(&source)) };
                        compose_arg(
                            arg_ty,
                            arg_composer.from(quote!(ffi_ref.#arg_0_name)),
                            Expression::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter([
                                            FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_composer.to(arg_to_conversion).present(&source)))
                                        ]).to_token_stream()).to_token_stream())),
                            arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                    }
                };
                // #constructor::new(
                let nested_ty = ty.first_nested_type().unwrap();
                let arg_0_presentation = compose(&arg_0_name, nested_ty);
                let nested_obj = source.maybe_object(nested_ty);
                let nested_opaque_obj = source.maybe_opaque_object(nested_ty);
                println!("AnyOther: {} obj: {}, opaque: {}", nested_ty.to_token_stream(), nested_obj.to_token_stream(), nested_opaque_obj.to_token_stream());
                // if nested_obj.is_none() {
                //     return FFIObjectPresentation::Empty.to_token_stream();
                // }
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(arg_0_name, FieldTypeConversionKind::Type(arg_0_presentation.ty))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_type.clone(), ty.clone()),
                            conversions: (
                                {
                                    let conversion = arg_0_presentation.from_conversion.present(&source);
                                    quote! {
                                        let ffi_ref = &*ffi;
                                        #constructor::new(#conversion)
                                    }
                                },
                                arg_0_presentation.to_conversion.present(&source),
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        },
                    ]),
                    Depunctuated::from_iter([arg_0_presentation.destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::Callback(ty) => {
                let type_path: TypePath = parse_quote!(#ty);
                let PathSegment { arguments, ..} = type_path.path.segments.last().unwrap();
                let ParenthesizedGenericArguments { inputs, output, .. } = parse_quote!(#arguments);
                let ffi_result = DictionaryName::FFiResult;
                let opt_conversion = |conversion: TokenStream2| quote! {
                    if ffi_result.is_null() {
                        None
                    } else {
                        #conversion
                    }
                };
                let from_complex_result = |ty: &Type, ffi_ty: &Type| quote! {
                    let result = <#ffi_ty as ferment_interfaces::FFIConversion<#ty>>::ffi_from(#ffi_result);
                    (self.destructor)(#ffi_result);
                    result
                };
                let from_opt_complex_result = |ty: &Type, ffi_ty: &Type| quote! {
                    let result = <#ffi_ty as ferment_interfaces::FFIConversion<#ty>>::ffi_from_opt(#ffi_result);
                    (self.destructor)(#ffi_result);
                    result
                };
                let from_primitive_result = || quote!(ffi_result);
                let from_opt_primitive_result = || DictionaryExpr::Deref(ffi_result.to_token_stream()).to_token_stream();
                let (return_type, ffi_return_type, post_processing) = match output {
                    ReturnType::Type(token, field_type) => {
                        // FromConversionComposer::
                        let full_ty: Type = field_type.resolve(&source);
                        let (ffi_ty, conversion) = match TypeConversion::from(&full_ty) {
                            TypeConversion::Primitive(_) => (full_ty.clone(), from_primitive_result()),
                            TypeConversion::Complex(ty) => {
                                let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                                let conversion = from_complex_result(&ty, &ffi_ty);
                                (ffi_ty.joined_mut(), conversion)
                            },
                            TypeConversion::Generic(generic_ty) => match generic_ty {
                                GenericTypeConversion::Optional(ty) => match TypeConversion::from(ty.first_nested_type().unwrap()) {
                                    TypeConversion::Primitive(ty) => (ty.joined_mut(), opt_conversion(from_opt_primitive_result())),
                                    TypeConversion::Complex(ty) => {
                                        let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                                        let conversion = opt_conversion(from_opt_complex_result(&ty, &ffi_ty));
                                        println!("Callback:: Return:: Generic:: Optional:: Complex:: {} -- {} -- {}", ty.to_token_stream(), ffi_ty.to_token_stream(), conversion);
                                        (ffi_ty.joined_mut(), conversion)
                                    },
                                    TypeConversion::Generic(ty) => {
                                        let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                                        let conversion = from_opt_complex_result(ty.ty().unwrap(), &ffi_ty);
                                        println!("Callback:: Return:: Generic:: Optional:: Generic:: {} -- {} -- {}", ty.to_token_stream(), ffi_ty.to_token_stream(), conversion);
                                        (ffi_ty.joined_mut(), conversion)
                                    },
                                },
                                GenericTypeConversion::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                _ => {
                                    let ffi_ty = full_ty.special_or_to_ffi_full_path_type(&source);
                                    let ty = generic_ty.ty().unwrap();
                                    let conversion = quote! {
                                        let result = <#ffi_ty as ferment_interfaces::FFIConversion<#ty>>::ffi_from(#ffi_result);
                                        (self.destructor)(#ffi_result);
                                        result
                                    };
                                    (ffi_ty.joined_mut(), conversion)
                                }
                            }
                        };
                        (
                            ReturnType::Type(token.clone(), Box::new(full_ty)),
                            ReturnType::Type(token.clone(), Box::new(ffi_ty)),
                            conversion
                        )
                    },
                    ReturnType::Default => (ReturnType::Default, ReturnType::Default, from_primitive_result()),
                };
                let mut args = CommaPunctuated::new();
                let mut arg_names = CommaPunctuated::new();
                let mut ffi_args = CommaPunctuated::new();
                let mut arg_target_types = CommaPunctuated::new();
                let mut arg_to_conversions = CommaPunctuated::new();
                inputs
                    .iter()
                    .enumerate()
                    .for_each(|(index, ty)| {
                        let conversion = TypeConversion::from(ty);
                        let name = Name::UnnamedArg(index);
                        arg_names.push(name.to_token_stream());
                        arg_target_types.push(ty.to_token_stream());
                        // args.push(ArgPresentation::NamedType { attrs: quote!(), name: name.to_token_stream(), var: ty.to_token_stream() });
                        args.push(ArgPresentation::Pat(Pat::Type(PatType {
                            attrs: vec![],
                            pat: Box::new(Pat::Verbatim(name.to_token_stream())),
                            colon_token: Default::default(),
                            ty: Box::new(ty.clone()),
                        })));
                        ffi_args.push(match &conversion {
                            TypeConversion::Primitive(ty) => ty.clone(),
                            TypeConversion::Complex(ty) => ty.special_or_to_ffi_full_path_variable_type(&source),
                            TypeConversion::Generic(generic_arg_ty) => if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                                match generic_arg_ty.ty() {
                                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                    Some(ty) => match TypeConversion::from(ty) {
                                        TypeConversion::Primitive(_) => ty.special_or_to_ffi_full_path_type(&source),
                                        TypeConversion::Generic(nested_nested) => nested_nested.special_or_to_ffi_full_path_type(&source),
                                        _ => ty.special_or_to_ffi_full_path_type(&source),
                                    }
                                }
                            } else {
                                generic_arg_ty.special_or_to_ffi_full_path_variable_type(&source)
                            },
                        }.to_token_stream());
                        arg_to_conversions.push(match &conversion {
                            TypeConversion::Primitive(..) => name.to_token_stream(),
                            TypeConversion::Generic(generic_ty) => match generic_ty {
                                GenericTypeConversion::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                GenericTypeConversion::Optional(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Primitive(_) => InterfacesMethodExpr::ToOptPrimitive(name.to_token_stream()).to_token_stream(),
                                    TypeConversion::Complex(_) |
                                    TypeConversion::Generic(_) => FFIConversionMethodExpr::FfiToOpt(name.to_token_stream()).to_token_stream(),
                                }
                                _ => FFIConversionMethodExpr::FfiTo(name.to_token_stream()).to_token_stream()
                            },
                            TypeConversion::Complex(..) => FFIConversionMethodExpr::FfiTo(name.to_token_stream()).to_token_stream(),
                        });
                    });
                let ffi_name = ty.mangle_ident_default();
                let ffi_type = ffi_name.to_type();
                FFIObjectPresentation::Generic {
                    object_presentation: create_callback(&ffi_name, &attrs, ffi_args.to_token_stream(), ffi_return_type),
                    interface_presentations: Depunctuated::from_iter([
                        InterfacePresentation::Callback {
                            attrs: attrs.clone(),
                            ffi_type: ffi_type,
                            inputs: args,
                            output: return_type,
                            body: quote! {
                                let ffi_result = (self.caller)(#arg_to_conversions);
                                #post_processing
                            }
                        }
                    ]),
                    drop_presentation: DropInterfacePresentation::Empty,
                    bindings: Default::default(),
                }
            },
            GenericTypeConversion::Optional(_) |
            GenericTypeConversion::Box(_) |
            GenericTypeConversion::TraitBounds(_) |
            _ => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}
fn compose_generic_group(ty: &Type, vec_conversion_type: Type, arg_conversion: TypeConversion, from_conversion_presentation: TokenStream2, to_conversion_presentation: TokenStream2, attrs: Vec<Attribute>, source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_name = ty.mangle_ident_default();
    let ffi_type = ffi_name.to_type();
    let arg_0_name = Name::Dictionary(DictionaryName::Values);
    let count_name = Name::Dictionary(DictionaryName::Count);
    let from_args = CommaPunctuated::from_iter([
        DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
        DictionaryExpr::SelfProp(count_name.to_token_stream())]);
    let arg_0_from = |composer: InterfacesMethodComposer|
        Expression::InterfacesExpr(composer(from_args.to_token_stream()));

    let arg_0_to = |composer: InterfacesMethodComposer|
        Expression::InterfacesExpr(
            InterfacesMethodExpr::Boxed(
                DictionaryExpr::SelfDestructuring(
                    CommaPunctuated::from_iter([
                        FieldComposer::named(count_name.clone(), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                        FieldComposer::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).to_token_stream()))])
                        .to_token_stream())
                    .to_token_stream()));

    let arg_0_destroy = |composer: InterfacesMethodComposer|
        Expression::InterfacesExpr(composer(from_args.to_token_stream()));

    let arg_0_presentation = match arg_conversion {
        TypeConversion::Primitive(arg_0_target_path) => {
            GenericArgPresentation::new(
                arg_0_target_path.clone(),
                arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                arg_0_from(FROM_PRIMITIVE_GROUP),
                arg_0_to(TO_PRIMITIVE_GROUP)
            )
        }
        TypeConversion::Complex(arg_0_target_ty) => {
            GenericArgPresentation::new(
                arg_0_target_ty.special_or_to_ffi_full_path_variable_type(source),
                arg_0_destroy(DESTROY_COMPLEX_GROUP),
                arg_0_from(FROM_COMPLEX_GROUP),
                arg_0_to(TO_COMPLEX_GROUP)
            )
        }
        TypeConversion::Generic(arg_0_generic_path_conversion) => {
            let (arg_0_composer, arg_ty) = {
                if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                    match arg_0_generic_path_conversion.ty() {
                        None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                        Some(ty) => match TypeConversion::from(ty) {
                            TypeConversion::Primitive(_) =>
                                (GenericArgComposer::new(FROM_OPT_PRIMITIVE_GROUP, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source)),
                            TypeConversion::Generic(nested_nested) => {
                                (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), nested_nested.special_or_to_ffi_full_path_variable_type(source))
                            },
                            _ => (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source) ),
                        }
                    }
                } else {
                    (GenericArgComposer::new(FROM_COMPLEX_GROUP, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(source))
                }
            };
            GenericArgPresentation::new(
                arg_ty,
                arg_0_destroy(arg_0_composer.destroy_composer),
                arg_0_from(arg_0_composer.from_composer),
                arg_0_to(arg_0_composer.to_composer)
            )
        }
    };
    compose_generic_presentation(
        ffi_name,
        attrs.clone(),
        Depunctuated::from_iter([
            FieldComposer::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
            FieldComposer::named(arg_0_name, FieldTypeConversionKind::Type(arg_0_presentation.ty.joined_mut()))
        ]),
        Depunctuated::from_iter([
            InterfacePresentation::Conversion {
                attrs: attrs.clone(),
                types: (ffi_type.clone(), ty.clone()),
                conversions: (
                    from_conversion_presentation,
                    to_conversion_presentation,
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                    None
                )
            },
            InterfacePresentation::VecConversion { attrs, types: (ffi_type, vec_conversion_type), decode: arg_0_presentation.from_conversion.present(source), encode: arg_0_presentation.to_conversion.present(source) }
        ]),
        Depunctuated::from_iter([arg_0_presentation.destructor.present(source).terminated()]),
        source
    )
}
pub fn compose_generic_presentation(
    ffi_name: Ident,
    attrs: Vec<Attribute>,
    field_conversions: Depunctuated<FieldComposer>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    FFIObjectPresentation::Generic {
        object_presentation: create_struct(
            &ffi_name,
            &attrs,
            BraceWrapped::new(
                CommaPunctuated::from_iter(field_conversions.iter()
                    .map(|field| OwnedItemPresentableContext::Named(field.clone(), Visibility::Public(VisPublic { pub_token: Default::default() })))))
            .present(source)),
        interface_presentations,
        drop_presentation: DropInterfacePresentation::Full {
            attrs: attrs.clone(),
            ty: ffi_name.to_type(),
            body: drop_body.to_token_stream()
        },
        bindings: compose_bindings(
            ffi_name.to_type(),
            attrs,
            None,
            field_conversions)
            .present(source)
    }
}

fn compose_bindings(
    ffi_type: Type,
    attrs: Vec<Attribute>,
    generics: Option<Generics>,
    conversions: Depunctuated<FieldComposer>
) -> Depunctuated<ConstructorBindingPresentableContext<Brace>> {
    Depunctuated::from_iter([
        struct_composer_ctor_root()((
            ConstructorPresentableContext::Default((ffi_type.clone(), attrs.clone(), generics.clone())),
            conversions.iter()
                .map(STRUCT_COMPOSER_CTOR_NAMED_ITEM)
                .collect())),
        BindingPresentableContext::Destructor(ffi_type.clone(), attrs, generics)
    ])
}

pub(crate) fn dictionary_generic_arg_pair(name: Name, field_name: Name, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty: Type = ty.resolve(source);
    let (destroy_expr, from_expr, to_expr) = match TypeConversion::from(&ty) {
        TypeConversion::Primitive(..) => (
            Expression::Empty,
            Expression::FfiRefWithName(name.clone()),
            Expression::ObjName(field_name).into()
        ),
        _ => (
            Expression::InterfacesExpr(DESTROY_COMPLEX(DictionaryExpr::SelfProp(name.to_token_stream()).to_token_stream())),
            Expression::From(Expression::FfiRefWithName(name.clone()).into()),
            Expression::To(Expression::ObjName(field_name).into())
        ),
    };
    (ty.clone(), Depunctuated::from_iter([GenericArgPresentation::new(ty, destroy_expr, from_expr, Expression::Named((name.to_token_stream(), to_expr.into())))]))
}

