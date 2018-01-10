extern crate argparse;
extern crate codevisual_web;
extern crate fs_extra;
extern crate open;
extern crate prelude;

#[allow(unused_imports)]
pub(crate) use prelude::*;

pub(crate) use std::path::Path;
pub(crate) use std::process::Command;
pub(crate) use std::fs::{self, File};

mod build;
mod run;

pub struct Options {
    pub release: bool,
    pub target: String,
    pub run: bool,
    pub sync_path: Option<String>,
    pub example: Option<String>,
    pub package: Option<String>,
    pub path: String,
    pub static_path: Option<String>,
    pub emsdk: Option<String>,
}

fn main() {
    let mut options = Options {
        release: true,
        target: "asmjs".into(),
        run: false,
        sync_path: None,
        example: None,
        package: None,
        path: ".".into(),
        emsdk: None,
        static_path: None,
    };
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.refer(&mut options.release).add_option(
            &["--debug"],
            argparse::StoreFalse,
            "Build with optimizations disabled",
        );
        ap.refer(&mut options.target)
            .add_option(&["--target"], argparse::Store, "asmjs | wasm32");
        ap.refer(&mut options.run).add_option(
            &["--run"],
            argparse::StoreTrue,
            "Run in browser after build",
        );
        ap.refer(&mut options.sync_path).add_option(
            &["--sync"],
            argparse::StoreOption,
            "Sync to remote host (specify full path)",
        );
        ap.refer(&mut options.example).add_option(
            &["--example"],
            argparse::StoreOption,
            "Example name, if some",
        );
        ap.refer(&mut options.package).add_option(
            &["--package"],
            argparse::StoreOption,
            "Package name, if some",
        );
        ap.refer(&mut options.path)
            .add_option(&["--path"], argparse::Store, "Path where to run");
        ap.refer(&mut options.emsdk).add_option(
            &["--emsdk"],
            argparse::StoreOption,
            "Path to emsdk",
        );
        ap.refer(&mut options.static_path).add_option(
            &["--static"],
            argparse::StoreOption,
            "Path to static data",
        );
        ap.parse_args_or_exit();
    }

    std::env::set_current_dir(&options.path).unwrap();

    build::build(&options);

    if let Some(ref sync_path) = options.sync_path {
        let mut command = Command::new("rsync");
        command.arg("-avz").arg("--delete");
        command.arg(format!("target/web/{}/*", options.target));
        command.arg(format!("{}/{}/codewars2017", sync_path, options.target));
        assert!(command.status().unwrap().success());
    }

    if options.run {
        run::run(&options);
    }
}
