fn main() {
    cxx_build::bridge("src/main.rs")
        .file("include/fibonacci.cpp")
        .flag_if_supported("-std=c++14")
        .compile("fibonacci");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=include/fibonacci.cpp");
    println!("cargo:rerun-if-changed=include/fibonacci.h");
}