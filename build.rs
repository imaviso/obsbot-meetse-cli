fn main() {
    println!("cargo:rerun-if-changed=wrapper/obsbot_wrapper.cpp");
    println!("cargo:rerun-if-changed=wrapper/obsbot_wrapper.h");

    // Link C++ standard library
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Link the SDK library
    // The library is in sdk/lib/
    // We need to tell rustc where to find it.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/sdk/lib", manifest_dir);
    println!("cargo:rustc-link-lib=dylib=dev");

    // Add rpath so the binary can find the .so at runtime without LD_LIBRARY_PATH
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/sdk/lib", manifest_dir);

    // Build the wrapper
    cc::Build::new()
        .cpp(true)
        .file("wrapper/obsbot_wrapper.cpp")
        .include("sdk/include")
        .compile("obsbot_wrapper");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper/obsbot_wrapper.h")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
