use std::process::Command;
use maud::html;

fn main() {
    // TODO: For Emscripten HTML / JS output
    /* 
    let name = "Lyra";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
    */

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

    /* 
    let projects = [
        "my_project1", 
        "my_project2", 
        "my_project3"
    ];

    for project in projects {
        let example = format!("{}::example", project);
        let status = Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(example)
            .status()
            .expect("Failed to execute cargo run");

        if !status.success() {
            println!("Failed to run example {}", example);
            break;
        }
    }
    */
}
