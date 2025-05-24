use std::{
    collections::HashMap,
    io::Error,
    process::{Command, Stdio},
};

pub fn init_cargo_project(project_name: &String) -> Result<(), Error> {
    let project_name_exist = std::path::Path::new(project_name).exists();

    if project_name_exist {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Project already exists",
        ));
    }

    let status = Command::new("cargo")
        .arg("new")
        .arg(project_name)
        .status()?;

    if !status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to create cargo project",
        ));
    }

    println!("✨ Cargo project successfully created!");

    Ok(())
}

pub fn add_basic_dependencies(project_name: &String) -> Result<(), Error> {
    let mut basic_dependencies = HashMap::new();
    basic_dependencies.insert("axum", None);
    basic_dependencies.insert("dotenv", None);
    basic_dependencies.insert("tokio", Some("full".to_string()));
    basic_dependencies.insert("serde", Some("derive".to_string()));
    basic_dependencies.insert("serde_json", None);
    basic_dependencies.insert("tower", None);
    basic_dependencies.insert("tower-http", Some("trace".to_string()));
    basic_dependencies.insert("tracing", None);
    basic_dependencies.insert("tracing-subscriber", None);

    for key in basic_dependencies.keys() {
        let mut command = Command::new("cargo");
        command.arg("add").arg(key);

        match basic_dependencies.get(key) {
            Some(Some(value)) => {
                command.arg("--features").arg(value);
            }
            _ => {}
        }

        let status = command
            .current_dir(project_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to add dependency: {}", key),
            ));
        }
    }

    println!("✅ Basic dependencies successfully added!");

    Ok(())
}

pub fn create_dirs(project_name: &String) -> Result<(), Error> {
    let dirs = vec!["routers", "models"];

    for dir in dirs {
        let dir_path = format!("{}/src/{}", project_name, dir);
        match std::fs::create_dir_all(dir_path) {
            Ok(_) => println!("✅ {} directory successfully created!", dir),
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
