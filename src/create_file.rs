use std::io::Error;

pub fn change_main(project_name: &String) -> Result<(), Error> {
    let main_source = include_str!("templates/main.rs");

    std::fs::write(format!("{}/src/main.rs", project_name), main_source)?;

    let fmt_status = std::process::Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("failed to execute process");

    if !fmt_status.status.success() {
        return Err(Error::new(std::io::ErrorKind::Other, "fmt failed"));
    }
    Ok(())
}

pub fn create_env_example_file(project_name: &String, server_url: &String) -> Result<(), Error> {
    if server_url == "0.0.0.0:3000" {
        let env_example_source = include_str!("templates/.env");
        std::fs::write(format!("{}/.env", project_name), env_example_source)?;
    } else {
        let mut env_file = std::fs::File::create(format!("{}/.env", project_name))?;
        let env_content = format!("SERVER_URL={}", server_url);
        std::io::Write::write_all(&mut env_file, env_content.as_bytes())?;
    }

    Ok(())
}
