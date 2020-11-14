extern crate bindgen;
extern crate proc_macro2;
extern crate syn;

use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};
use std::path::Path;
use std::path::PathBuf;
use syn::export::ToTokens;

/// Get the function bindings that will either need to be attached to a dynamic library object
/// or re-created into an `extern "C"` block.
fn get_binding_functions(file: &mut syn::File) -> Vec<syn::ForeignItemFn> {
	let mut retvl = Vec::new();
	// Iterate in reverse order by index so we can modify in-place
	for idx in (0..file.items.len()).rev() {
		// Just to be safe, also verify that they really are extern "C" blocks.
		let should_get = match file.items.get(idx) {
			Some(syn::Item::ForeignMod(m)) => {
				m.abi.name.as_ref().map_or(false, |l| &l.value() == "C")
			}
			_ => false,
		};
		if !should_get {
			continue;
		}
		let mut binding = match file.items.remove(idx) {
			syn::Item::ForeignMod(m) => m,
			_ => unreachable!(),
		};

		// Currently do not support generated function attributes.
		assert!(binding.attrs.is_empty());
		for item in binding.items.drain(..) {
			match item {
				syn::ForeignItem::Fn(bind_fn) => {
					retvl.push(bind_fn);
				}
				other => {
					// Currently do not support statics.
					unimplemented!("Found non-function binding {:?}", other);
				}
			}
		}
	}
	retvl
}

/// Generates the function binding code.
///
/// In the case of a dynamic library, this creates a struct called `LibraryWrapper` with a single field, `inner`,
/// of type `libloading::Library`. Initially, it is also created with a constructor,
/// `from_path(path : impl AsRef<std::ffi::OsStr>) -> Result<Self, libloading::Error>`, to load a shard object at the given path.
///
/// The functions are then transformed from raw `extern "C" fn name(inputs) -> outputs`
/// into functions of the form `pub unsafe fn name(&self, inputs) -> Result<outputs, libloading::Error>`.
///
/// The error case is for the case that `libloading` cannot find the symbol.
fn construct_bindings(raw_bindings: Vec<syn::ForeignItemFn>, should_dyn: bool) -> Vec<syn::Item> {
	if !should_dyn {
		// Case static library: just wrap in an `extern "C"` block.
		let mut wrapper: syn::ItemForeignMod = syn::parse_quote!(
			extern "C" {}
		);
		for binding in raw_bindings {
			wrapper.items.push(syn::ForeignItem::Fn(binding));
		}
		vec![syn::Item::ForeignMod(wrapper)]
	} else {
		// The struct declaration of the dynamic library wrapper
		let library_struct_wrapper: syn::Item = syn::parse_quote!(
			#[derive(Clone)]
			pub struct LibraryWrapper {
				inner: std::sync::Arc<libloading::Library>,
			}
		);

		// The `impl` block, initialized with the constructor
		let mut impl_wrapper: syn::ItemImpl = syn::parse_quote!(
			impl LibraryWrapper {
				pub fn from_path(path : impl AsRef<std::ffi::OsStr>) -> Result<Self, libloading::Error> {
					let inner = std::sync::Arc::new(libloading::Library::new(&path)?);
					Ok(Self { inner })
				}
			}
		);

		for func in raw_bindings {
			let output_type = match func.sig.output {
				syn::ReturnType::Default => Box::new(syn::parse_quote!(())),
				syn::ReturnType::Type(_, inner) => inner,
			};
			let inpt = func.sig.inputs;
			let name = func.sig.ident;
			let name_str =
				proc_macro2::Literal::byte_string(format!("{}\0", name.to_string()).as_bytes());

			let input_types = inpt
				.pairs()
				.filter_map(|pair| {
					let arg: &syn::FnArg = pair.value();
					match arg {
						syn::FnArg::Receiver(_) => None,
						syn::FnArg::Typed(t) => Some(t.ty.to_owned()),
					}
				})
				.collect::<syn::punctuated::Punctuated<_, syn::token::Comma>>();
			let input_names = inpt
				.pairs()
				.filter_map(|pair| {
					let arg: &syn::FnArg = pair.value();
					match arg {
						syn::FnArg::Receiver(_) => None,
						syn::FnArg::Typed(t) => Some(t.pat.to_owned()),
					}
				})
				.collect::<syn::punctuated::Punctuated<_, syn::token::Comma>>();
			let bd: syn::ImplItemMethod = syn::parse_quote!(
				pub unsafe fn #name (&self, #inpt) -> Result<#output_type, libloading::Error> {
					let dyn_symbol = self.inner.get::<unsafe extern fn(#input_types) -> #output_type>(#name_str)?;
					Ok( dyn_symbol(#input_names) )
				}
			);
			impl_wrapper.items.push(syn::ImplItem::Method(bd));
		}
		vec![library_struct_wrapper, syn::Item::Impl(impl_wrapper)]
	}
}

fn write_to_file(path: impl AsRef<Path>, bindings: &bindgen::Bindings, dynamic: bool) {
	let mut file: syn::File = syn::parse_str(&bindings.to_string()).unwrap();
	let externs = get_binding_functions(&mut file);
	let mapped = construct_bindings(externs, dynamic);
	file.items.extend(mapped.into_iter());
	let output_text = add_spacings(file.into_token_stream());
	std::fs::write(path, output_text.as_bytes()).unwrap();
}

// Workaround for https://github.com/rust-lang/rust-bindgen/issues/1600
fn add_spacings(tokens: TokenStream) -> String {
	let mut res_str = String::new();
	struct TokenVisitor<'s> {
		s: &'s mut String,
	}
	impl<'s> TokenVisitor<'s> {
		fn visit_stream(&mut self, stream: TokenStream) {
			let mut iter = stream.into_iter().peekable();
			while let Some(tree) = iter.next() {
				self.visit_tree(tree, iter.peek().is_some());
			}
		}
		fn visit_tree(&mut self, tree: TokenTree, put_space: bool) {
			let mut put_space = put_space;
			match tree {
				TokenTree::Group(group) => match group.delimiter() {
					Delimiter::Brace => {
						*self.s += "{\n";
						self.visit_stream(group.stream());
						*self.s += "\n}\n";
					}
					_ => *self.s += &group.to_string(),
				},
				TokenTree::Ident(ident) => {
					*self.s += &ident.to_string();
				}
				TokenTree::Punct(punct) => {
					if punct.spacing() == Spacing::Alone && punct.as_char() == ';' {
						*self.s += &punct.to_string();
						*self.s += "\n";
					} else {
						*self.s += &punct.to_string();
					}
					if punct.spacing() == Spacing::Joint {
						put_space = false;
					}
				}
				TokenTree::Literal(l) => {
					*self.s += &l.to_string();
				}
			}
			if put_space {
				*self.s += " ";
			}
		}
	}
	let mut visitor = TokenVisitor { s: &mut res_str };
	visitor.visit_stream(tokens);
	res_str
}

fn main() {
	let bindings = bindgen::Builder::default()
		.header("stddef.h")
		.header("sys/deepspeech/native_client/deepspeech.h")
		.clang_args(&["-x", "c++", "-std=c++11"])
		.generate()
		.expect("Couldn't generate bindings");

	let out_path = PathBuf::from("sys/src");
	write_to_file(out_path.join("bindings.rs"), &bindings, false);
	write_to_file(PathBuf::from("src/dynamic_bindings.rs"), &bindings, true);
}
