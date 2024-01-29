use std::path::{Path, PathBuf};

// TODO: Make these programmatic / templated
const PACKAGES: [&str; 2] = ["snake_engine", "snake"];
const TARGET: &str = "wasm32-unknown-emscripten";
const DEBUG: bool = false;
const PORT: i16 = 5000;

fn generate_html() -> Result<(), Box<dyn std::error::Error>> {
    use maud::html;
    let name = "Rou";
    let _markup = html! {
        p { "Hi, " (name) "!" }
    };
    // println!("Generated HTML: {}", markup.into_string());
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
            let mut source_path = PathBuf::from(&exe_path);
            source_path.push(TARGET);
            source_path.push(debug);
            source_path.push(format!("{}{}", package, file_ext));
        
            // Output path for projects that will be run
            // and libraries that will be dynamically linked at runtime
            let mut destination_path = PathBuf::from(&package_path);
            destination_path.push("dist");

            // Create the dist directory if it does not exist
            std::fs::create_dir_all(&destination_path)?;

            destination_path.push(format!("{}{}", package, file_ext));
        
            match std::fs::copy(&source_path, &destination_path) {
                Ok(_) => (),
                Err(_e) => panic!("Failed to copy file {} to {}", source_path.display(), destination_path.display()),
            }
        }
    }

    // Copy the index file from the templates directory to the dist directory
    let root_path = env!("WORKSPACE_DIR", "Failed to retrieve workspace root path");
    let index_src = Path::new(&root_path).join("templates/index.html");
    let index_dst = Path::new(&package_path).join("dist/index.html");
    std::fs::copy(&index_src, &index_dst)?;
    
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
                    // Dev / Debug
                    "-g",
                    "-O0", 
                    // Nessecary for all profiles
                    "-sMAIN_MODULE=2", 
                    "-sFORCE_FILESYSTEM=1",
                    "-sERROR_ON_UNDEFINED_SYMBOLS=0", 
                    "-sEXPORT_ES6=1", 
                    "-sMODULARIZE=1", 
                    "-sUSE_ES6_IMPORT_META=1", 
                    "-sEXPORTED_RUNTIME_METHODS=[_free,_malloc,allocateUTF8,UTF8ToString,writeArrayToMemory,FS,loadDynamicLibrary]", 
                    "-sALLOW_MEMORY_GROWTH=1",
                    "-sMIN_WEBGL_VERSION=2",
                    "-sFETCH=1",
                    "-Wno-unused-command-line-argument",
                    // "-sUSE_PTHREADS=1", 
                    // "-sPTHREAD_POOL_SIZE=4",
                    // "-sWEBSOCKET_DEBUG",
                    "-lwebsocket.js",
                    // "-sUSE_SDL=2",
                    // "-sUSE_SDL_IMAGE=2",
                    // "-sEXCEPTION_CATCHING_ALLOWED",
                    // "-sASSERTIONS",
                    // "-sDISABLE_EXCEPTION_CATCHING=0"
                    // "--preload-file assets",
                    ];
                // command.env("RUSTFLAGS", "-C target-feature=+atomics,+bulk-memory");
                command.env("EMCC_CFLAGS", flags.join(" "));
                // command.env("EMCC_FORCE_STDLIBS", "1");
                // command.env("EMCC_FORCE_STDLIBS", "libmalloc,libc++,libc++abi,libsockets,libfetch");
                // command.env("EMCC_DEBUG", "1");
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

fn watch() -> Result<(), Box<dyn std::error::Error>> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    let package_path = env!("CARGO_MANIFEST_DIR", "Failed to retrieve package path");
    let package_path = std::path::Path::new(package_path)
        .parent()
        .ok_or("Could not get package path parent")?;

    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(package_path, RecursiveMode::Recursive)?;

    // print all events, non returning
    println!(
        "Watching project files for changes at... {}",
        package_path.display()
    );

    for res in rx {
        match res {
            Ok(event) => {
                if !event.paths.is_empty() {
                    println!("Rebuilding... Changed file: {:?}", event.paths[0].display());
                    watcher.unwatch(package_path)?;
                    build_packages()?;
                    copy_files()?;
                    watcher.watch(package_path, RecursiveMode::Recursive)?;
                } else {
                    println!("Build failed {:?}", event)
                }
            }
            Err(error) => println!("watch error: {:?}", error),
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_packages()?;
    copy_files()?;
    if TARGET.contains("emscripten") {
        generate_html()?;
        std::thread::spawn(move || {
            serve().unwrap();
        });
        watch()?;
    }
    Ok(())
}
