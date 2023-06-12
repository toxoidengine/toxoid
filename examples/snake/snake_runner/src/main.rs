// TODO: Make these programmatic / templated
const PACKAGES: [&str; 2] = ["snake_engine", "snake"];
const TARGET: &str = "wasm32-unknown-emscripten";
const DEBUG: bool = false;
const PORT: i16 = 5000;

fn generate_html() -> Result<(), Box<dyn std::error::Error>> {
    use maud::html;
    let name = "Rou";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("Generated HTML: {}", markup.into_string());
    Ok(())
}

fn copy_files() -> Result<(), Box<dyn std::error::Error>> {
    let package_path = env!("CARGO_MANIFEST_DIR", "Failed to retrieve package path");
    let package_path = std::path::Path::new(package_path)
        .parent()
        .ok_or("Could not get package path parent")?
        .to_str()
        .ok_or("Could not convert package path parent to string")?;
    let exe_path = std::env::current_exe()?;
    let exe_path = exe_path
        .parent()
        .ok_or("Could not get executable path parent")?
        .parent() // Go up one more level to get to the target directory
        .ok_or("Could not get executable path parent's parent")?
        .to_str()
        .ok_or("Could not convert executable path parent to string")?;
    let debug = if DEBUG { "debug" } else { "release" };
    println!(
        "\nPackage Path: {}, \nExecutable Path: {}, \nBuild Type: {}",
        package_path, exe_path, debug
    );
    for package in PACKAGES {
        let file_exts = if TARGET.contains("emscripten") {
            if package.contains("engine") {
                // If engine it will have a emscripten JS file for WASM loading in
                // the browser
                vec![".js", ".wasm"]
            } else {
                // If a dynamically linked WASM module, it will only have a .wasm file
                // and no .js file since it's not an application
                vec![".wasm"]
            }
        } else if TARGET.contains("windows") {
            vec![".exe"]
        } else {
            vec![""] // Assuming non-Windows and non-WASM operating systems have no file extension
        };

        for file_ext in file_exts {
            // Source path for projects that will be run
            let source_path = format!("{}/{}/{}/{}{}", exe_path, TARGET, debug, package, file_ext);
            // Output path for projects that will be run
            // and libraries that will be dynamically linked at runtime
            let destination_path = format!("{}/dist/{}{}", package_path, package, file_ext);

            println!("Source Path: {}", source_path);
            println!("Destination Path: {}", destination_path);

            match std::fs::copy(source_path, destination_path) {
                Ok(_) => println!("File copied successfully."),
                Err(e) => eprintln!("Failed to copy file: {}", e),
            }
        }
    }
    Ok(())
}

fn build_packages() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    for package in PACKAGES {
        let mut command = Command::new("cargo");
        command
            .arg("build")
            .arg("--package")
            .arg(package)
            .arg("--target")
            .arg(TARGET);
        if !DEBUG || TARGET.contains("emscripten") {
            command.arg("--release");
        };
        if TARGET.contains("emscripten") {
            if package.contains("engine") {
                // If Emscripten target engine, set Emscripten environment variables for main module
                let flags = [
                    "-O0", 
                    "-sMAIN_MODULE=2", 
                    "-sFORCE_FILESYSTEM=1",
                    "-sERROR_ON_UNDEFINED_SYMBOLS=0", 
                    "-sEXPORT_ES6=1", 
                    "-sMODULARIZE=1", 
                    "-sUSE_ES6_IMPORT_META=1", 
                    "-sEXPORTED_RUNTIME_METHODS=[_free,_malloc,allocateUTF8,UTF8ToString,writeArrayToMemory,FS,loadDynamicLibrary]", 
                    "-sALLOW_MEMORY_GROWTH=1",
                    "-sUSE_SDL=2",
                    "-sUSE_SDL_IMAGE=2",
                    "-sMIN_WEBGL_VERSION=2",
               ];
                command.env("EMCC_CFLAGS", flags.join(" "));
            } else {
                // If emscripten target library, set Emscripten environment variables for library / side module
                // That will be dynamically linked at runtime
                command.env("EMCC_CFLAGS", "-sSIDE_MODULE=2");
            }
        }
        command
            .status()
            .expect(format!("Failed to run package {}", package).as_str());
    }
    Ok(())
}

fn serve() -> Result<(), Box<dyn std::error::Error>> {
    use tiny_file_server::FileServer;
    println!("Serving project files at port {}", PORT);
    let package_path = env!("CARGO_MANIFEST_DIR", "Failed to retrieve package path");
    let package_path = std::path::Path::new(package_path)
        .parent()
        .ok_or("Could not get package path parent")?
        .to_str()
        .ok_or("Could not convert package path parent to string")?;
    let destination_path = format!("{}/dist", package_path);
    FileServer::http(format!("127.0.0.1:{}", PORT))
        .expect("Failed to start HTTP server")
        .run(destination_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_packages()?;
    copy_files()?;
    if TARGET.contains("emscripten") {
        generate_html()?;
        serve()?;
    }
    Ok(())
}
