use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, Item, ItemTrait, ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use crate::composition::{GenericConversion, NestedArgument, TraitDecompositionPart1, TypeComposition};
use crate::Config;
use crate::context::{Scope, ScopeChain, TypeChain};
use crate::context::custom_resolver::CustomResolver;
use crate::context::generic_resolver::GenericResolver;
use crate::context::import_resolver::ImportResolver;
use crate::conversion::{ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::formatter::format_global_context;
use crate::holder::PathHolder;
use crate::context::{ScopeResolver, TraitsResolver};
use crate::context::scope_resolver::ScopeRefinement;
use crate::ext::{CrateExtension, Pop, RefineMut, RefineUnrefined, Unrefined};
use crate::helper::{collect_bounds, GenericExtension};

#[derive(Clone)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    // crate::asyn::query::Query: [T: [TransportRequest]]
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    pub imports: ImportResolver,

    pub refined_generics: HashSet<GenericConversion>
}

impl std::fmt::Debug for GlobalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_global_context(self))
    }
}

impl std::fmt::Display for GlobalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<&Config> for GlobalContext {
    fn from(config: &Config) -> Self {
        GlobalContext::with_config(config.clone())
    }
}
impl GlobalContext {
    pub fn with_config(config: Config) -> Self {
        Self { config, scope_register: ScopeResolver::default(), generics: Default::default(), traits: Default::default(), custom: Default::default(), imports: Default::default(), refined_generics: HashSet::new() }
    }
    pub fn fermented_mod_name(&self) -> &str {
        &self.config.mod_name
    }
    pub fn is_fermented_mod(&self, ident: &Ident) -> bool {
        format_ident!("{}", self.fermented_mod_name()).eq(ident)
    }


    pub fn resolve_trait_type(&self, from_type: &Type) -> Option<&ObjectConversion> {
        // println!("resolve_trait_type: {}", from_type.to_token_stream());
        // RESOLVE PATHS
        // Self::asyn::query::TransportRequest::Client::Error
        // ? [Self::asyn::query::TransportRequest::Client::Error] Self
        // : [Self::asyn::query::TransportRequest::Client] Self::Error
        // : [Self::asyn::query::TransportRequest] Self::Client::Error
        //  : [Self::asyn::query::TransportRequest] Self::Client -> [Self::asyn::query::TransportClient] Self::Error

        // aa::bb::cc::dd::ee
        // 1. a) [aa::bb::cc::dd::ee] Self
        // 2. a) [aa::bb::cc::dd] Self::ee
        // 3. a) [aa::bb::cc::dd] Self, [Self::ee]
        // 4. a) [aa::bb::cc] Self::dd::ee, b) [aa::bb::cc] Self::dd
        let current_scope: PathHolder = parse_quote!(#from_type);
        let mut i = 0;
        let mut maybe_trait: Option<&ObjectConversion>  = None;
        while i < current_scope.len() && maybe_trait.is_none() {
            let (root, head) = current_scope.split_and_join_self(i);
            let ty = parse_quote!(#head);
            let root_scope = self.maybe_scope(&root.0);
            if let Some(scope) = root_scope {
                maybe_trait = self.maybe_scope_object(&ty, scope);
            }
            //maybe_trait = self.maybe_scope_type(&ty, &root);
            if i > 0 {
                match maybe_trait {
                    Some(ObjectConversion::Item(TypeCompositionConversion::Trait(_trait_ty, decomposition, _super_bounds), _)) |
                    Some(ObjectConversion::Type(TypeCompositionConversion::Trait(_trait_ty, decomposition, _super_bounds))) => {
                        let ident = &head.0.segments.last().unwrap().ident;
                        // println!("FFI (has decomposition) for: {}: {}", format_token_stream(ident), trait_ty);
                        if let Some(trait_type) = decomposition.types.get(ident) {
                            // println!("FFI (first bound) {:?}", trait_type);
                            if let Some(first_bound) = trait_type.trait_bounds.first() {
                                // println!("FFI (first bound) {}", format_token_stream(&first_bound.path));
                                let tt_type = parse_quote!(#first_bound);
                                if let Some(scope) = root_scope {
                                    maybe_trait = self.maybe_scope_object(&tt_type, scope);
                                }
                                // println!("FFI (first bound full) {:?}", maybe_trait);
                            }
                        }
                    },
                    _ => {}
                }
            }
            // println!("FFI (resolve....) for: {} in [{}] ===> {:?}", format_token_stream(&head), format_token_stream(&root), maybe_trait);
            i += 1;
        }
        maybe_trait
    }


    // pub fn maybe_scope_generic_bounds_or_parent(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
    //     // println!("maybe_scope_generic_bounds_or_parent: {} in [{}]...", ident, scope);
    //     self.generics.maybe_generic_bounds(scope, ident)
    //         .and_then(|generic_bounds| {
    //             let first_bound = generic_bounds.first().unwrap();
    //             let first_bound_as_scope = PathHolder::from(first_bound);
    //             self.maybe_import(scope, &first_bound_as_scope)
    //         })
    // }



    fn maybe_obj_or_parent_scope_type(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_object(ty, self_scope)
            .or(match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_scope_object(ty, parent_chain),
                _ => None,
            })
    }

    pub fn maybe_fn_type(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_object(ty, fn_scope).or(match parent_scope {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                self.maybe_scope_object(ty, parent_scope),
            ScopeChain::Fn { parent_scope_chain, .. } =>
                self.maybe_fn_type(parent_scope, parent_scope_chain, ty),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } =>
                self.maybe_scope_object(ty, parent_scope).or(match &**parent_scope_chain {
                    ScopeChain::CrateRoot { .. } |
                    ScopeChain::Mod { ..} =>
                        self.maybe_scope_object(ty, &parent_scope_chain),
                    _ => None,
                }),
        })
    }

    pub fn maybe_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
         match scope {
             ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                 self.maybe_scope_object(ty, &scope),
             ScopeChain::Fn { parent_scope_chain, .. } =>
                 self.maybe_fn_type(scope, parent_scope_chain, ty),
             ScopeChain::Trait { parent_scope_chain, .. } |
             ScopeChain::Object { parent_scope_chain, .. } |
             ScopeChain::Impl { parent_scope_chain, .. } =>
                 self.maybe_obj_or_parent_scope_type(scope, parent_scope_chain, ty),
         }
    }

    pub fn maybe_item(&self, path: &Path) -> Option<&ScopeItemConversion> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope(path) {
            // println!("maybe_item: found scope: {}", scope);
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = parse_quote!(#last_ident);
            if let Some(ObjectConversion::Item(_, item)) = self.maybe_type(&ty, scope) {
                // println!("maybe_item: found item in scope: {}", item);
                return Some(item);
            } else {
                println!("[WARN] Scope found [{}] but no item: {}", scope, path.to_token_stream());
            }
        }
        None
    }

    pub fn find_scopes_for_path(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.resolve(path)
    }

    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> ScopeChain {
        // println!("actual_scope_for_type: {} in [{}]", format_token_stream(ty), current_scope);
        let p = parse_quote!(#ty);
        let scope = if let Some(st) = self.maybe_scope_object(ty, current_scope) {
            // match st {
            //     ObjectConversion::Type(_) => {}
            //     ObjectConversion::Item(_, _) => {}
            //     ObjectConversion::Empty => {}
            // }
            let self_ty = st.to_ty().unwrap();
            let self_path: Path = parse_quote!(#self_ty);
            self.maybe_scope(&self_path).cloned()
        } else if let Some(import_path) = self.maybe_scope_import_path(current_scope, &p) {
            self.maybe_scope(import_path).cloned()
        } else {
            None
        };
        // println!("actual_scope_for_type: [{:?}]", scope);
        scope.unwrap_or(ScopeChain::crate_root(current_scope.crate_ident().clone()))
    }
    // pub fn maybe_trait(&self, full_ty: &Type) -> Option<TraitCompositionPart1> {
    //     let full_scope: PathHolder = parse_quote!(#full_ty);
    //     self.maybe_scope(&full_scope.0)
    //         .and_then(|scope| self.traits.maybe_trait(scope).cloned())
    // }


    // fn find_trait_item_full_paths_pair(&self, ) -> (Scope, Scope) {
    //     self.used_traits_dictionary.iter()
    //         .for_each(|(item_full_path, trait_path_chunks)| {
    //             trait_path_chunks.iter()
    //                 .for_each(|trait_ident_or_chunk| {
    //                     let trait_ident = trait_ident_or_chunk.root_ident();
    //                     // Restore full trait path using imports
    //                     // TODO: can be chunk so need to handle not only idents
    //                     let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, &trait_ident) {
    //                         Scope::from(import)
    //                     } else {
    //                         item_full_path.popped().joined(&trait_ident)
    //                     };
    //                     (trait_scope, item_full_path)
    //                 });
    //
    //         });
    //
    // }

    // pub fn inject_types_from_traits_implementation(&mut self) {
    //     let self_tc = TypeHolder::new(parse_quote!(Self));
    //     self.used_traits_dictionary.iter()
    //         .for_each(|(item_full_path, trait_path_chunks)| {
    //             trait_path_chunks.iter()
    //                 .for_each(|trait_ident_or_chunk| {
    //                     let trait_ident = trait_ident_or_chunk.root_ident();
    //                     // Restore full trait path using imports
    //                     // TODO: can be chunk so need to handle not only idents
    //                     let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, &trait_ident) {
    //                         Scope::from(import)
    //                     } else {
    //                         item_full_path.popped().joined(&trait_ident)
    //                     };
    //                     println!("inject_types_from_traits_implementation: [{}]: {}: {}", format_token_stream(item_full_path), format_token_stream(trait_ident_or_chunk), format_token_stream(&trait_scope));
    //                     if let Some(types_used_by_trait) = self.scope_types.get(&trait_scope).cloned() {
    //                         // Copy them to implementor's types
    //                         println!("copy types except self:\n   {}", format_types_dict(&types_used_by_trait));
    //                         // TODO: do we need to replace Self to <Self as #trait>?
    //                         let types = types_used_by_trait.into_iter().filter(|(tc, tyty)| {
    //
    //                             !self_tc.eq(tc)
    //                         });
    //                         self.scope_types.entry(item_full_path.clone())
    //                             .or_default()
    //                             .extend(types);
    //                     }
    //             });
    //
    //     });
    // }
    // pub fn inject_types_from_traits_implementation(&mut self) {
    //     let self_tc: TypeHolder = parse_quote!(Self);
    //
    //     // Collect necessary data in a Vec to avoid borrowing `self` while iterating.
    //     let mut trait_data = Vec::new();
    //
    //     for (item_full_path, trait_path_chunks) in &self.used_traits_dictionary {
    //         for trait_ident_or_chunk in trait_path_chunks {
    //
    //             // let trait_ident = trait_ident_or_chunk.root_ident();
    //             let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, trait_ident_or_chunk) {
    //                 PathHolder::from(import)
    //             } else {
    //                 // item_full_path.parent_scope()
    //                 item_full_path.parent_scope().joined_path(trait_ident_or_chunk.0.clone())
    //             };
    //             println!("inject_types_from_traits_implementation: [{}]: {}: {}", format_token_stream(item_full_path), format_token_stream(trait_ident_or_chunk), format_token_stream(&trait_scope));
    //
    //             if let Some(types_used_by_trait) = self.scope_types.get(&trait_scope) {
    //                 trait_data.push((item_full_path.clone(), types_used_by_trait.clone()));
    //             }
    //         }
    //     }
    //
    //     // Now, iterate over the collected data and modify `self.scope_types`.
    //     for (item_full_path, types_used_by_trait) in trait_data {
    //         // println!("copy types except self:\n   {}", format_types_dict(&types_used_by_trait));
    //         let types = types_used_by_trait.into_iter().filter(|(tc, _)| !self_tc.eq(tc));
    //         self.scope_types
    //             .entry(item_full_path)
    //             .or_default()
    //             .extend(types);
    //     }
    // }
}



/// Imports
impl GlobalContext {
    pub fn maybe_scope_import_path(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        self.imports.maybe_path(scope, ident)
    }

    pub fn maybe_import(&self, scope: &ScopeChain, path: &PathHolder) -> Option<&Path> {
        let result_opt = self.imports.maybe_import(scope, path);
        println!("maybe_import: {path} in [{}] ---> {}", scope.self_path_holder(), result_opt.map_or(quote!(None), |r| r.to_token_stream()));
        result_opt
    }

    fn maybe_known_item(&self, ty_to_replace: &TypeComposition) -> Option<TypeCompositionConversion> {
        let ty = &ty_to_replace.ty;
        let path_to_replace: Path = parse_quote!(#ty);
        // println!("find_the_item: {}", path_to_replace.to_token_stream());
        if let Some(item) = self.maybe_item(&path_to_replace) {
            //println!("find_the_item: found: {} --- {}", ty_to_replace.ty.to_token_stream(), item);
            match item {
                ScopeItemConversion::Item(item) => match item {
                    Item::Trait(ItemTrait { ident, items, supertraits, .. }) =>
                        Some(TypeCompositionConversion::Trait(
                            ty_to_replace.clone(),
                            TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits))),
                    Item::Enum(..) |
                    Item::Struct(..) |
                    Item::Type(..) |
                    Item::Fn(..) |
                    Item::Impl(..) =>
                        Some(TypeCompositionConversion::Object(ty_to_replace.clone())),
                    _ => None
                }
                ScopeItemConversion::Fn(_) => None
            }
        } else {
            None
        }
    }

    fn maybe_refined_object(&self, scope: &ScopeChain, object: &ObjectConversion) -> Option<ObjectConversion> {
        match object {
            ObjectConversion::Type(TypeCompositionConversion::Imported(ty_composition, import_path)) => {
                let mut ty_replacement = ty_composition.clone();
                let mut import_type_path: TypePath = parse_quote!(#import_path);
                let last_segment_pair = import_type_path.path.segments.pop().unwrap();
                if import_path.is_crate_based() {
                    import_type_path.path = import_path.replaced_first_with_ident(&scope.crate_ident_as_path());
                } else {
                    import_type_path.path = import_path.clone();
                };
                let mut chunks = import_type_path.path.clone();
                while !chunks.segments.is_empty() {
                    chunks.segments = chunks.segments.popped();
                    if !chunks.segments.is_empty() {
                        let scope = create_mod_chain(&chunks);
                        if let Some(parent_imports) = self.imports.maybe_scope_imports(&scope) {
                            for (PathHolder(_ident), alias_path) in parent_imports {
                                if let Some(merged) = merged_import(&import_type_path.path, alias_path) {
                                    import_type_path.path.segments = merged.segments;
                                }
                            }
                        }
                    }
                }

                let mut last_segment2 = last_segment_pair.into_value();

                match &mut last_segment2.arguments {
                    PathArguments::None => {}
                    PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                        panic!("Parenthesized args: {} -> {}", inputs.to_token_stream(), output.to_token_stream())
                    },
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                        println!("GENERIC::Args:: {}", args.to_token_stream());
                        args.iter_mut()
                            .for_each(|arg| match arg {
                                GenericArgument::Type(inner_ty) => {
                                    println!("GENERIC::TYpe:: {}", inner_ty.to_token_stream());
                                    match ty_replacement.nested_arguments.pop() {
                                        None => {}
                                        Some(nested_arg) => match nested_arg.into_value() {
                                            NestedArgument::Object(obj) => {
                                                *inner_ty = obj.to_ty().unwrap();
                                            }
                                        }
                                    }
                                }
                                GenericArgument::Lifetime(_) => {}
                                GenericArgument::Const(_) => {}
                                GenericArgument::Binding(_) => {}
                                GenericArgument::Constraint(_) => {}
                            });
                    }
                };

                import_type_path.path.segments.last_mut().unwrap().arguments = last_segment2.arguments;
                //println!("replaced import: {} --> {}", import_path.to_token_stream(), type_path.to_token_stream());
                // determine the actual type
                let dict_path = import_type_path.path.clone();
                ty_replacement.ty = Type::Path(import_type_path);
                // let conversion_replacement = scope.maybe_dictionary_type(&dict_path, self)
                //     .or(self.maybe_known_item(&ty_replacement))
                //     .unwrap_or(TypeCompositionConversion::Unknown(ty_replacement));

                let conversion_replacement = if let Some(dictionary_type) = scope.maybe_dictionary_type(&dict_path, self) {
                    dictionary_type
                    // } else if let Some(custom) = self.custom.maybe_conversion(&ty_replacement.ty) {
                    //
                } else if let Some(found_item) = self.maybe_known_item(&ty_replacement) {
                    // println!("[INFO] known item for [{}] is [{}]", ty_replacement.ty.to_token_stream(), found_item.to_token_stream());
                    found_item
                } else {
                    println!("[WARN] Still unknown -> {}", ty_replacement.ty.to_token_stream());
                    TypeCompositionConversion::Unknown(ty_replacement)
                };
                return Some(ObjectConversion::Type(conversion_replacement));
            },
            ObjectConversion::Type(TypeCompositionConversion::Unknown(ty_to_replace)) => {
                if let Some(refined) = self.maybe_known_item(ty_to_replace) {
                    // println!("Type clarified: [{}] ----> is [{}]", ty_composition.ty.to_token_stream(), clarified);
                    return Some(ObjectConversion::Type(refined));
                }
            },
            ObjectConversion::Type(TypeCompositionConversion::Tuple(ty_composition)) => {
                // println!("CLARIFY TUPLE.1 {}", ty_composition);
                let mut new_ty_composition = ty_composition.clone();
                match &mut new_ty_composition.ty {
                    Type::Tuple(type_tuple) => {
                        type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                            match &mut new_ty_composition.nested_arguments[index] {
                                NestedArgument::Object(obj) => {
                                    if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                                        let to_ty = object_to_refine.to_ty().unwrap();
                                        // println!("TUPLE ITEM REFINED: {} --> {} --> {}", obj, object_to_refine, to_ty.to_token_stream());
                                        *obj = object_to_refine;
                                        *elem = to_ty;
                                    }
                                },
                            }
                        });
                    },
                    _ => {}
                }
                // println!("CLARIFY TUPLE.2 {}", new_ty_composition);
                return Some(ObjectConversion::Type(TypeCompositionConversion::Tuple(new_ty_composition)));
            },
            ObjectConversion::Type(TypeCompositionConversion::Object(composition)) => {
                return Some(ObjectConversion::Type(TypeCompositionConversion::Object(self.refine_nested(composition, scope))));
            },
            ObjectConversion::Type(TypeCompositionConversion::Trait(composition, dec, paths)) => {
                return Some(ObjectConversion::Type(TypeCompositionConversion::Trait(self.refine_nested(composition, scope), dec.clone(), paths.clone())));
            },
            ObjectConversion::Type(TypeCompositionConversion::TraitType(composition)) => {
                return Some(ObjectConversion::Type(TypeCompositionConversion::TraitType(self.refine_nested(composition, scope))));
            },
            ObjectConversion::Type(TypeCompositionConversion::Bounds(_composition)) => {
                // TODO::
            },
            ObjectConversion::Type(TypeCompositionConversion::SmartPointer(_composition)) => {
                // TODO::
            },
            ObjectConversion::Type(TypeCompositionConversion::Fn(_composition)) => {
                // TODO::
            },
            _ => {}
        }
        None
    }
    fn refine_nested(&self, composition: &TypeComposition, scope: &ScopeChain) -> TypeComposition {
        let mut new_ty_composition = composition.clone();
        new_ty_composition.nested_arguments
            .iter_mut()
            .for_each(|arg| match arg {
                NestedArgument::Object(obj) => {
                    if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                        *obj = object_to_refine;
                    }
                }
            });
        new_ty_composition.ty.refine_with(new_ty_composition.nested_arguments.clone());
        new_ty_composition
    }
}

impl RefineMut for GlobalContext {
    type Refinement = ScopeRefinement;
    fn refine_with(&mut self, refined: Self::Refinement) {
        self.scope_register.refine_with(refined);
        let mut refined_generics = HashSet::new();
        self.scope_register.inner.values()
            .for_each(|type_chain| {
                type_chain.inner.values().for_each(|conversion| {
                    match conversion {
                        ObjectConversion::Type(ty) => {
                            refined_generics.extend(ty.to_ty().find_generics()
                                .iter()
                                .filter_map(|holder| type_chain.find(&holder))
                                .map(GenericConversion::from));
                        },
                        ObjectConversion::Item(_, conversion) => {
                            refined_generics.extend(conversion.find_generics_fq(type_chain));
                        }
                        ObjectConversion::Empty => {}
                    }
                })
            });
        self.refined_generics = refined_generics;
    }
}

impl Unrefined for GlobalContext {
    type Unrefinement = ScopeRefinement;
    fn unrefined(&self) -> Self::Unrefinement {
        let mut scope_updates = vec![];
        // let mut generic_updates = vec![];
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_types_to_refine = type_chain.inner.iter()
                    .filter_map(|(holder, object)|
                        if let Some(object_to_refine) = self.maybe_refined_object(scope, object) {
                            Some((holder.clone(), object_to_refine))
                        } else {
                            None
                        })
                    .collect::<HashMap<_, _>>();
                // let generics_to_refine = type_chain.inner.iter().filter_map(|(holder, object)| {
                //
                // }).collect();

                if !scope_types_to_refine.is_empty() {
                    scope_updates.push((scope.clone(), scope_types_to_refine));
                }
            });

        //self.scope_register.inner.iter().for_each()

        // let mut iter = scope_updates.iter()
        //     .map(|(key, value)| format!("\t{}:\n\t\t{}", key, format_types_dict(&value)))
        //     .collect::<Vec<String>>();
        // iter.sort();
        //
        // println!("scope_updates: \n{}", iter.join("\n\n"));
        scope_updates
    }
}

impl RefineUnrefined for GlobalContext {}

/// Scope
impl GlobalContext {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        let result = self.scope_register.scope_register_mut(scope);
        // println!("scope_mut: {} -> \n{}", scope, result);
        result
    }
    pub fn maybe_scope_object(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        self.scope_register.maybe_scope_type(ty, scope)
    }
    pub fn maybe_scope(&self, path: &Path) -> Option<&ScopeChain> {
        let x = self.scope_register.resolve(path);
        // println!("maybe_scope: {} --> {}", path.to_token_stream(), x.map_or(format!("None"), |x| x.to_string()));
        x
    }

}

pub fn create_mod_chain(path: &Path) -> ScopeChain {
    // print!("create_mod_chain: {}", path.to_token_stream());
    let segments = &path.segments;

    let crate_ident = &segments.first().unwrap().ident;
    let self_scope = Scope::new(PathHolder::from(path), ObjectConversion::Empty);
    let parent_chunks = path.popped();
    let parent_scope_chain = if parent_chunks.segments.len() > 1 {
        create_mod_chain(&parent_chunks)
    } else {
        ScopeChain::CrateRoot {
            crate_ident: crate_ident.clone(),
            self_scope: Scope { self_scope: PathHolder(parent_chunks), object: ObjectConversion::Empty }
        }
    };
    if segments.len() == 1 {
        ScopeChain::CrateRoot {
            crate_ident: crate_ident.clone(),
            self_scope
        }
    } else {
        ScopeChain::Mod {
            crate_ident: crate_ident.clone(),
            self_scope,
            parent_scope_chain: Box::new(parent_scope_chain.clone())
        }
    }

}
pub fn merged_import(import_path: &Path, alias: &Path) -> Option<Path> {
    let mut last_import_segments = import_path.clone();
    let mut last_alias_segments = alias.clone();
    let mut merged_path: Option<Path> = None;
    // println!("merged_import: {} <===> {}", import_path.to_token_stream(), alias.to_token_stream());
    match (last_import_segments.segments.pop(), last_alias_segments.segments.pop()) {
        (Some(last_import_pair), Some(last_alias_pair)) => {
            let ident = last_import_pair.value().ident.clone();
            if ident.eq(&last_alias_pair.value().ident) {
                let mut path = Path { leading_colon: None, segments: last_import_segments.segments };
                path.segments.extend(last_alias_segments.segments);
                path.segments.push(PathSegment::from(ident));
                println!("merged_path: {}", path.to_token_stream());
                merged_path = Some(path);
            }
        },
        _ => {}
    }
    merged_path
}
