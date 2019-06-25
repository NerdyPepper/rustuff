#![allow(unused_imports)]
use cargo::core::dependency::Kind;
use cargo::core::manifest::ManifestMetadata;
use cargo::core::package::PackageSet;
use cargo::core::registry::PackageRegistry;
use cargo::core::resolver::Method;
use cargo::core::shell::Shell;
use cargo::core::{Package, PackageId, Resolve, Workspace};
use cargo::ops;
use cargo::util::{self, important_paths, CargoResult, Cfg, Rustc};
use cargo::{CliResult, Config};
use std::path::PathBuf;

fn main() {
    let config = Config::default().unwrap();
    let manifest_path = PathBuf::from("/home/nerdypepper/code/taizen/Cargo.toml");

    let ws = Workspace::new(&manifest_path, &config).unwrap();

    println!("{:?}", ws.root());
    for member in ws.members() {
        let pkg_manifest = member.manifest();
        println!("{}", pkg_manifest.name());
        println!("{}", pkg_manifest.version());
        println!("{:?}", pkg_manifest.links());
        println!("{}", pkg_manifest.edition());


    }
}
