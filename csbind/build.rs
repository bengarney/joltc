use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    bindgen::Builder::default()
        .header("../include/joltc.h")
        .generate()
        .unwrap()
        .write_to_file("src/joltc.rs")
        .unwrap();

    cc::Build::new()
        .cpp(true)
        .shared_flag(true)
        .flag("-std=c++17")
        .files([
            "../src/joltc.c",
            "../src/joltc_assert.cpp",
            "../src/joltc.cpp",
        ])
        .include("../include")
        .include("../build/_deps/joltphysics-src/")
        .compile("joltc");

csbindgen::Builder::default()
        .input_bindgen_file("src/joltc.rs")
        // .method_filter(|x| x.starts_with("LZ4"))
        .rust_method_prefix("csjoltphysics_")
        // .rust_file_header("use super::lz4;")
        // .rust_method_type_path("lz4")
        .csharp_class_name("Jolt")
        .csharp_namespace("JoltPhysicsC")
        .csharp_dll_name("JoltPhysicsC")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        // .csharp_entry_point_prefix("joltc_")
        .csharp_method_prefix("")
        .csharp_class_accessibility("public")
        //.csharp_c_long_convert("int")
        //.csharp_c_ulong_convert("uint")
        // .csharp_use_function_pointer(true)
        .csharp_generate_const_filter(|_| true)
        // .generate_to_file("src/lz4_ffi.rs", "../dotnet-sandbox/lz4_bindgen.cs")
        .generate_to_file("src/joltc_ffi.rs", "joltc_bindgen.cs")
        .unwrap();

    Ok(())
}