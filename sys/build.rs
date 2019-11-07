extern crate bindgen;
extern crate proc_macro2;

use std::env;
use std::path::PathBuf;
use std::path::Path;

// Workaround for https://github.com/rust-lang/rust-bindgen/issues/1600
fn write_to_file(path :impl AsRef<Path>, bindings :&bindgen::Bindings) {
	use proc_macro2::{TokenStream, TokenTree, Spacing, Delimiter};
	use std::str::FromStr;
	let str_bindings = bindings.to_string();
	let tokens = TokenStream::from_str(&str_bindings).unwrap();
	let mut res_str = String::new();
	struct TokenVisitor<'s> {
		s: &'s mut String,
	}
	impl<'s> TokenVisitor<'s> {
		fn visit_stream(&mut self, stream :TokenStream) {
			let mut iter = stream.into_iter().peekable();
			while let Some(tree) = iter.next() {
				self.visit_tree(tree, iter.peek().is_some());
			}
		}
		fn visit_tree(&mut self, tree :TokenTree, put_space :bool) {
			let mut put_space = put_space;
			match tree {
				TokenTree::Group(group) => {
					match group.delimiter() {
						Delimiter::Brace => {
							*self.s += "{\n";
							self.visit_stream(group.stream());
							*self.s += "\n}\n";
						},
						_ => *self.s += &group.to_string(),
					}
				},
				TokenTree::Ident(ident) => {
					*self.s += &ident.to_string();
				},
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
				},
				TokenTree::Literal(l) => {
					*self.s += &l.to_string();
				},
			}
			if put_space {
				*self.s += " ";
			}
		}
	}
	let mut visitor = TokenVisitor {
		s: &mut res_str
	};
	visitor.visit_stream(tokens);
	std::fs::write(path, visitor.s.as_bytes()).unwrap();
}

fn main() {
	println!("cargo:rustc-link-lib=deepspeech");

	let bindings = bindgen::Builder::default()
		.header("stddef.h")
		.header("deepspeech/native_client/deepspeech.h")
		.clang_args(&["-x", "c++", "-std=c++11"])
		.generate()
		.expect("Couldn't generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	write_to_file(out_path.join("bindings.rs"), &bindings);
}
