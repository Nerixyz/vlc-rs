use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let libvlc_location = get_libvlc();
    if let Some((lib_dir, include_dir)) = libvlc_location.as_ref() {
        println!("cargo:rustc-link-search={}", lib_dir.to_string_lossy());
        println!("cargo:include={}", include_dir.to_string_lossy());
    }

    println!("cargo:rustc-link-lib=dylib=libvlc");

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // For no_std
        .use_core()
        // Use libc
        .ctypes_prefix("libc")
        // Whitelist
        .whitelist_type(".*vlc.*")
        .whitelist_function(".*vlc.*")
        .whitelist_var(".*vlc.*")
        .whitelist_function(".*vsnprintf.*")
        .rustified_enum("libvlc_meta_t")
        .rustified_enum("libvlc_state_t")
        .rustified_enum("libvlc_track_type_t")
        .rustified_enum("libvlc_position_t")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    if let Some((_, include_dir)) = libvlc_location {
        bindings = bindings.clang_arg(format!("-I{}", include_dir.to_string_lossy()));
    }

    if cfg!(windows) {
        // windows doesn't have the "ssize_t" so we'll try to polyfill it
        bindings = bindings.clang_arg("-Dssize_t=int64_t")
    }

    // Set header include paths
    if let Ok(pkg_config_library) = pkg_config::Config::new().probe("libvlc") {
        for include_path in &pkg_config_library.include_paths {
            bindings = bindings.clang_arg(format!("-I{}", include_path.display()));
        }
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn get_libvlc() -> Option<(PathBuf, PathBuf)> {
    if let Ok(nuget_dir) = env::var("LIBVLC_NUGET_DIR").map(PathBuf::from) {
        // + LIBVLC_NUGET_DIR
        //   + libvlc.lib
        //   + include/
        return Some((nuget_dir.clone(), nuget_dir.join("include")));
    }
    let lib_dir = env::var("LIBVLC_LIB_DIR").map(PathBuf::from).ok();
    let include_dir = env::var("LIBVLC_INCLUDE_DIR").map(PathBuf::from).ok();

    Some(match (lib_dir, include_dir) {
        (Some(lib_dir), Some(include_dir)) => (lib_dir, include_dir),
        (lib_dir, include_dir) => {
            // + LIBVLC_DIR
            //    + lib/
            //    + include/
            let libvlc_dir = env::var("LIBVLC_DIR").ok()?;
            let libvlc_dir = Path::new(&libvlc_dir);
            let lib_dir = lib_dir.unwrap_or_else(|| libvlc_dir.join("lib"));
            let include_dir = include_dir.unwrap_or_else(|| libvlc_dir.join("include"));
            (lib_dir, include_dir)
        }
    })
}
