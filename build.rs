use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let clblas_cmake = PathBuf::from("clBLAS")
        .join("src");
    let dst = cmake::Config::new(clblas_cmake)
        .define("BUILD_TEST", "OFF")
        .define("BUILD_KTEST", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").join("import").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib64").join("import").display()
    );
    println!("cargo:rustc-link-lib=static=clBLAS");
    
    let bindings = bindgen::Builder::default()
        .raw_line("#![allow(warnings)]")
        .raw_line("use cl_sys::*;")
        .clang_arg("-std=c++03")
        .header("wrapper.hpp")
        .clang_arg("-I")
        .clang_arg(dst.join("include").display().to_string())
        .emit_builtins()
        .generate_block(false)
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .rustified_non_exhaustive_enum("clblas.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to create bindings.");
    bindings.write_to_file("src/lib.rs")?;
    
    println!("cargo:rustc-link-lib=stdc++");
    
    println!("cargo:rustc-rerun-if-changed=wrapper.hpp");
    
    Ok(())
}
