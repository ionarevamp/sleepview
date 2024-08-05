fn main() {
    #[cfg(feature = "gold")]
    println!("cargo:rustc-link-arg=-fuse-ld=gold");
    #[cfg(feature = "mold")]
    {
        println!("cargo:rustc-linker=clang");
        println!("cargo:rustc-link-arg=-fuse-ld=mold");
    }
}
