fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=native=linked");
    println!("cargo:rustc-link-lib=dylib=geode");
    println!("cargo:rustc-link-lib=dylib=XInput1_4");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .clang_arg("-I./includes")
        .clang_arg("-I./loader/include")
        .clang_arg("-I./loader/include/Geode/cocos/extensions")
        .clang_arg("-I./loader/include/Geode/cocos/include")
        .clang_arg("-I./loader/include/Geode/fmod")
        .clang_arg("-I./build/bindings/bindings")
        .clang_arg("-DGEODE_MOD_ID=\"examplemod\"")
        .clang_arg("-Wno-microsoft-enum-forward-reference")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(".*")
        .allowlist_file(".*")
        .generate_inline_functions(true)
        .generate_comments(true)
        .layout_tests(false)
        .derive_debug(true)
        .derive_copy(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_ord(true)
        .generate_block(true)
        .derive_partialord(true)
        .derive_partialeq(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .rustified_enum(".*")
        .size_t_is_usize(true)
        .c_naming(false)
        .vtable_generation(true)
        .sort_semantically(true)
        .wrap_unsafe_ops(true)
        .translate_enum_integer_types(true)
        .enable_cxx_namespaces()
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
