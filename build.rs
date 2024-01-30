fn main() {
    #[cfg(target_os = "windows")]
    {
        let lib_dir = r"C:\Users\jmfra\scoop\apps\sdl2\current\lib";

        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=static=SDL2");
    }
}
