extern crate curl;
extern crate gl_generator;

use curl::easy::Easy;
use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const SDL2_FILENAME: &'static str = "SDL2-devel-2.28.2-VC.zip";
const SDL2_URL: &'static str = "https://www.libsdl.org/release/SDL2-devel-2.28.2-VC.zip";
const SDL2_PATH: &'static str = "SDL2-devel-2.28.2-VC";

fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=build.rs");

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (2, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    // Download SDL if needed
    if !Path::new(SDL2_FILENAME).exists() {
        download_from_url(SDL2_URL, SDL2_FILENAME);
    }

    if !Path::new(SDL2_PATH).exists() {
        unzip_file(SDL2_FILENAME);
    }
}

fn download_from_url(url: &str, dst_file: &str) {
    File::create(dst_file)
        .and_then(|mut file| {
            let mut curl = Easy::new();
            curl.url(url).expect("Error setting url");
            curl.write_function(move |data| Ok(file.write(data).expect("Error writing data")))
                .expect("Error setting write function");
            curl.perform().expect("Error downloading archive");
            Ok(())
        })
        .expect("Could not open output file");
}

fn unzip_file(filename: &str) {
    let file = std::fs::File::open(filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Create a new directory with the same name as the zip file
    let dir_name = filename.strip_suffix(".zip").unwrap();
    std::fs::create_dir_all(dir_name).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // Append the file name to the new directory path
        let outpath = Path::new(dir_name).join(file.name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}