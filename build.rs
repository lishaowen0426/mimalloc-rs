use std::env;
use std::format;
use std::path::{Path, PathBuf};
use std::process::Command;

const MIMALLOC_ROOT: &str = "/home/swli/inprocess/mimalloc";
const MIMALLOC_SRC: &str = "/home/swli/inprocess/mimalloc/src";

fn rerun_if_changed_anything_in_dir(dir: &Path) {
    let mut stack = dir
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| &*e.file_name() != ".git")
        .collect::<Vec<_>>();
    while let Some(entry) = stack.pop() {
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            stack.extend(path.read_dir().unwrap().map(|e| e.unwrap()));
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn run() {
    let mut out_dir = PathBuf::new();
    out_dir.push(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("mimalloc/build");
    let install_dir = out_dir.join("mimalloc/install");
    Command::new("cmake")
        .args(["-S", MIMALLOC_ROOT])
        .args(["-B", build_dir.to_str().unwrap()])
        .args([
            "-DMI_SECURE=ON",
            "-DMI_BUILD_SHARED=OFF",
            "-DMI_BUILD_OBJECT=ON",
            "-DMI_BUILD_TESTS=OFF",
            "-DMI_INSTALL_TOPLEVEL=ON",
            "-DCMAKE_BUILD_TYPE=Release",
        ])
        .arg(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            install_dir.to_str().unwrap()
        ))
        .output()
        .expect("mimalloc cmake configure failed");

    Command::new("make")
        .args(["-C", build_dir.to_str().unwrap()])
        .arg("install")
        .output()
        .expect("make mimalloc filed");

    let lib_dir = install_dir.join("lib");
    println!(
        "cargo::rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=mimalloc");
}
fn main() {
    rerun_if_changed_anything_in_dir(Path::new(MIMALLOC_SRC));
    run();
}
