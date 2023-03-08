use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em"); // (not currently used)
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_base");
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");
    }

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }

    let board_name = env::vars_os()
        .map(|(a, _)| a.to_string_lossy().to_string())
        .find(|x| x.starts_with("CARGO_FEATURE_BOARD+"))
        .map(|s| {
            s.strip_prefix("CARGO_FEATURE_BOARD+")
                .unwrap()
                .to_ascii_lowercase()
                .replace('_', "-")
        });

    if let Some(board_name) = board_name {
        let data_dir = PathBuf::from(format!("src/boards/{}", board_name));
        let in_memory_x = std::fs::read(data_dir.join("memory.x")).unwrap();

        gen_memory(&board_name, &in_memory_x[..]);
    }

    pub fn gen_memory(board_name: &str, in_memory_x: &[u8]) {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        std::fs::create_dir_all(format!("{}/boards/{}", out_dir.display(), board_name)).unwrap();
        let out_memory_x = format!("{}/boards/{}/memory.x", out_dir.display(), board_name);
        File::create(&out_memory_x)
            .unwrap()
            .write_all(in_memory_x)
            .unwrap();

        println!("cargo:rerun-if-changed=build.rs");
        println!(
            "cargo:rustc-link-search={}/boards/{}",
            out_dir.display(),
            board_name
        );
    }
}
