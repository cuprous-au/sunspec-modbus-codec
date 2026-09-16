//! `#[derive(ModelList)]` for `sunspec_modbus_lib_rs::ModelList`.
//!
//! Derives a [`ModelList`](https://docs.rs/sunspec-modbus-lib-rs) impl for a struct of SunSpec models - either tuple or
//! named. This includes:
//!  * a ReadAdapters struct covering every model,
//!  * a WriteAdapters struct covering only the models that are writable,
//!  * traversable Iterator impls for each of the above that pair each model with its adapter, traversed in the order of
//!    definition in the source struct

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Index, Member, Path, parse_macro_input};

mod model_registry;

struct ModelField {
    name: String,
    /// How this field is accessed on `self` - its name for a named struct, its position for a
    /// tuple struct. Every field also appears in the generated `ReadAdapters` in the same order
    /// (and, for a named struct, under the same name), so this doubles as the read-side adapter
    /// access too; `WriteAdapters` only carries the writable subset, so a tuple struct's
    /// write-side access is renumbered separately - see `derive_model_list`.
    self_member: Member,
    /// The field type's path with its last segment removed, e.g. `model_708` - where
    /// `ReadAdapter`/`WriteAdapter` live for this model.
    module: Path,
    /// The field type's last path segment, e.g. `Model708` - also the name of its
    /// `ReadBinding`/`WriteBinding` variant.
    variant: Ident,
    /// Whether this model has any writable points at all.
    writable: bool,
}

fn resolve_field(index: usize, field: &syn::Field) -> syn::Result<ModelField> {
    let self_member = match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(Index::from(index)),
    };

    let syn::Type::Path(type_path) = &field.ty else {
        return Err(syn::Error::new_spanned(
            &field.ty,
            "ModelList fields must name a model type, e.g. `model_1::Model1`",
        ));
    };

    let mut module = type_path.path.clone();
    let Some(variant_segment) = module.segments.pop() else {
        return Err(syn::Error::new_spanned(
            &field.ty,
            "ModelList fields must name a model type, e.g. `model_1::Model1`",
        ));
    };
    // `Punctuated::pop` removes the last segment but leaves its separating `::` behind.
    module.segments.pop_punct();
    let variant = variant_segment.into_value().ident;

    let Some(module_name) = module
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return Err(syn::Error::new_spanned(
            &field.ty,
            "ModelList fields must be qualified by their model module, e.g. `model_1::Model1`",
        ));
    };

    let writable = model_registry::WRITABLE_MODELS.contains(&module_name.as_str());

    Ok(ModelField {
        name: module_name,
        self_member,
        module,
        variant,
        writable,
    })
}

#[proc_macro_derive(ModelList)]
pub fn derive_model_list(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let vis = &input.vis;

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(struct_name, "ModelList can only be derived for structs")
            .to_compile_error()
            .into();
    };
    let (field_nodes, tuple) = match &data.fields {
        Fields::Named(fields) => (&fields.named, false),
        Fields::Unnamed(fields) => (&fields.unnamed, true),
        Fields::Unit => {
            return syn::Error::new_spanned(
                struct_name,
                "ModelList can only be derived for structs with fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = match field_nodes
        .iter()
        .enumerate()
        .map(|(index, field)| resolve_field(index, field))
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(fields) => fields,
        Err(error) => return error.to_compile_error().into(),
    };

    if fields.is_empty() || fields[0].name != "model_1" {
        return syn::Error::new_spanned(
            &field_nodes[0],
            "First model in a ModelList must be an instance of Model 1",
        )
        .to_compile_error()
        .into();
    }

    let base_name = struct_name
        .to_string()
        .strip_suffix("Models")
        .map(str::to_owned)
        .unwrap_or_else(|| struct_name.to_string());
    let read_adapters_name = format_ident!("{base_name}ReadAdapters");
    let write_adapters_name = format_ident!("{base_name}WriteAdapters");

    let read_adapter_fields = fields.iter().map(|field| {
        let module = &field.module;
        if tuple {
            quote! { #vis &'a dyn #module::ReadAdapter }
        } else {
            let name = &field.self_member;
            quote! { #vis #name: &'a dyn #module::ReadAdapter }
        }
    });

    let write_adapter_fields = fields.iter().filter(|field| field.writable).map(|field| {
        let module = &field.module;
        if tuple {
            quote! { #vis &'a mut dyn #module::WriteAdapter }
        } else {
            let name = &field.self_member;
            quote! { #vis #name: &'a mut dyn #module::WriteAdapter }
        }
    });

    let read_bindings = fields.iter().map(|field| {
        let ModelField {
            self_member,
            variant,
            ..
        } = field;
        quote! {
            ::sunspec_modbus_lib_rs::sunspec::adapters::ReadBinding::#variant(&self.#self_member, adapters.#self_member)
        }
    });

    // Only the writable subset ends up in `WriteAdapters`. A named struct's fields keep their
    // own names regardless of which siblings are dropped, so write-side access reuses
    // `self_member` there too; a tuple struct's fields are repositioned around the subset, so
    // its write-side access is renumbered here, counting only the writable fields seen so far.
    let mut next_write_index: usize = 0;
    let write_bindings = fields.iter().map(|field| {
        let ModelField {
            self_member,
            variant,
            writable,
            ..
        } = field;
        if !*writable {
            return quote! {
                ::sunspec_modbus_lib_rs::sunspec::adapters::WriteBinding::#variant(&self.#self_member)
            };
        }
        let adapter_member: Member = if tuple {
            Member::Unnamed(Index::from(next_write_index))
        } else {
            self_member.clone()
        };
        next_write_index += 1;
        quote! {
            ::sunspec_modbus_lib_rs::sunspec::adapters::WriteBinding::#variant(&self.#self_member, adapters.#adapter_member)
        }
    }).collect::<Vec<_>>();

    let read_adapters_def = if tuple {
        quote! {
            #[doc(hidden)]
            #vis struct #read_adapters_name<'a>(#(#read_adapter_fields),*);
        }
    } else {
        quote! {
            #[doc(hidden)]
            #vis struct #read_adapters_name<'a> {
                #(#read_adapter_fields),*
            }
        }
    };

    let write_adapters_def = if tuple {
        quote! {
            #[doc(hidden)]
            #vis struct #write_adapters_name<'a>(#(#write_adapter_fields),*);
        }
    } else {
        quote! {
            #[doc(hidden)]
            #vis struct #write_adapters_name<'a> {
                #(#write_adapter_fields),*
            }
        }
    };

    let expanded = quote! {
        #read_adapters_def
        #write_adapters_def

        impl ::sunspec_modbus_lib_rs::ModelList for #struct_name {
            type ReadAdapters<'a> = &'a #read_adapters_name<'a>;
            type WriteAdapters<'a> = &'a mut #write_adapters_name<'a>;

            fn read_iter<'a>(
                &'a self,
                adapters: Self::ReadAdapters<'a>,
            ) -> impl Iterator<Item = ::sunspec_modbus_lib_rs::sunspec::adapters::ReadBinding<'a>> {
                [#(#read_bindings),*].into_iter()
            }

            fn write_iter<'a>(
                &'a self,
                adapters: Self::WriteAdapters<'a>,
            ) -> impl Iterator<Item = ::sunspec_modbus_lib_rs::sunspec::adapters::WriteBinding<'a>> {
                [#(#write_bindings),*].into_iter()
            }
        }
    };
    expanded.into()
}
