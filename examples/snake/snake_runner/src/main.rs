use std::process::Command;

// TODO: Make this programmatic / templated
const PACKAGES: [&str; 2] = [
    "snake_engine", 
    "snake"
];
// TODO: Make this programmatic / templated
const TARGET: &str = "wasm32-unknown-emscripten"; 

fn generate_html() {
    use maud::html;
    // TODO: For Emscripten HTML / JS output
    let name = "Lyra";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
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
        .to_str()
        .ok_or("Could not convert executable path parent to string")?; 
    println!("\nPackage Path: {}, \nExecutable Path: {}", package_path, exe_path);
    for package in PACKAGES {
        let file_exts = if cfg!(all(target_arch = "wasm32", target_os = "emscripten")) {
            if package.contains("engine") {
                // If engine it will have a emscripten JS file for WASM loading in
                // the browser
                vec![".js", ".wasm"]
            } else {
                // If a dynamically linked WASM module, it will only have a .wasm file
                // and no .js file since it's not an application
                vec![".wasm"]
            }
        } else if cfg!(target_os = "windows") {
            vec![".exe"]
        } else {
            vec![""] // Assuming non-Windows and non-WASM operating systems have no file extension
        };
        
        for file_ext in file_exts {
            // Source path for projects that will be run
            let source_path = format!(
                "{}/target/{}/debug/{}{}", 
                exe_path,
                TARGET,
                package,
                file_ext
            );
            // Output path for projects that will be run 
            // and libraries that will be dynamically linked at runtime
            let destination_path = format!(
                "{}/dist/{}{}", 
                package_path,
                package,
                file_ext
            );
        
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
    for package in PACKAGES {
        Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg(package)
            // TODO: Conditional compilation here
            .arg("--target")
            .arg("wasm32-unknown-emscripten")
            .status()
            .expect(format!("Failed to run package {}", package).as_str());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    build_packages()?;
    copy_files()?;
    Ok(())
}
