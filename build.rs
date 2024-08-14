fn main() {
    #[cfg(all(feature = "gold", not(target_os = "windows")))]
    {
        println!("cargo:rustc-link-arg=-fuse-ld=gold");

        #[cfg(all(feature = "cortex-a8", feature = "arm1176"))]
        compile_error!("`cortex-a8` and `arm1176` features can not be enabled together, as they enable fixes for different processors.");
        #[cfg(feature = "cortex-a8")]
        println!("cargo:rustc-link-arg=-Wl,--fix-cortex-a8");
        #[cfg(feature = "arm1176")]

        println!("cargo:rustc-link-arg=-Wl,--fix-arm1176");
        println!("cargo:rustc-link-arg=-Wl,--gc-sections,--icf=all,-O3");
    }
    #[cfg(feature = "mold")]
    {
        println!("cargo:rustc-linker=clang");
        println!("cargo:rustc-link-arg=-fuse-ld=mold");
        println!("cargo:rustc-link-arg=-Wl,--gc-sections,--icf=all,-O3");

    }

    println!("cargo:rustc-link-arg=-Os");
}
