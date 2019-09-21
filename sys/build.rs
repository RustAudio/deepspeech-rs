use std::env;
use std::path::PathBuf;

fn main() {
	println!("cargo:rustc-link-lib=deepspeech");

	let bindings = bindgen::Builder::default()
		.header("stddef.h")
		.header("deepspeech/native_client/deepspeech.h")
		.clang_args(&["-x", "c++", "-std=c++11"])
		.generate()
		.expect("Couldn't generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
