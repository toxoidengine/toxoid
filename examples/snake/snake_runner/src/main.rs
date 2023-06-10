use std::process::Command;

fn generate_html() {
    use maud::html;
    // TODO: For Emscripten HTML / JS output
    let name = "Lyra";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
}

fn copy_files() {
    let root_path = "./"; // TODO: Make this programmatic
    let file_ext = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        "" // Assuming non-Windows operating systems have no file extension
    };

    let package_name = std::env::var("CARGO_PKG_NAME")
        .expect("Failed to retrieve package name");
    let source_path = format!(
        "{}target/debug/{}{}", 
        root_path, 
        package_name,
        file_ext
    );
    let destination_path = format!(
        "{}static/{}{}", 
        root_path,
        package_name,
        file_ext
    );

    println!("source_path: {}", source_path);
    println!("destination_path: {}", destination_path);

    match std::fs::copy(source_path, destination_path) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => eprintln!("Failed to copy file: {}", e),
    }
}

fn main() {
    // TODO: Make this programmatic
    let packages = [
        "snake_engine", 
        "snake"
    ];

    for package in packages {
        let status = Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg(package)
            // TODO: Conditional compilation here
            .arg("--target")
            .arg("wasm32-unknown-emscripten")
            .status()
            .expect("Failed to execute cargo run");

        if !status.success() {
            println!("Failed to run pacakge {}", package);
            break;
        }
    }
}
