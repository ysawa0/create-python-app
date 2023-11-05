extern crate regex;

use askama::Template;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::process;

#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
struct GitIgnore {}

#[derive(Template)]
#[template(path = "Makefile", escape = "none")]
struct Makefile {}

#[derive(Template)]
#[template(path = "Dockerfile", escape = "none")]
struct Dockerfile {}

#[derive(Template)]
#[template(path = "main.py", escape = "none")]
struct MainPy {}

#[derive(Template)]
#[template(path = ".pre-commit-config.yaml", escape = "none")]
struct PreCommitConfig {
    python: bool,
}

#[derive(Template)]
#[template(path = "pyproject.toml", escape = "none")]
struct PyProject {
    name: String,
    python_ver: String,
    black_target_ver: String,
}

#[derive(Template)]
#[template(path = ".cpa/flake8.cfg", escape = "none")]
struct Flake8 {}

#[derive(Template)]
#[template(path = ".cpa/prettier.json", escape = "none")]
struct Prettier {}

pub fn setup_preset(mut preset: String, name: String) {
    if preset == "python" {
        preset = "python3.10".to_string();
    }

    let _ = fs::create_dir_all(format!("./{}/.cpa", name));
    // Render .gitignore
    File::create(format!("./{}/.gitignore", name))
        .and_then(|mut file| {
            file.write_all(
                GitIgnore {}
                    .render()
                    .expect("Failed to render .gitignore")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to .gitignore");

    // Render Makefile
    File::create(format!("./{}/Makefile", name))
        .and_then(|mut file| {
            file.write_all(
                Makefile {}
                    .render()
                    .expect("Failed to render Makefile")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to Makefile");

    // Render Dockerfile
    File::create(format!("./{}/Dockerfile", name))
        .and_then(|mut file| {
            file.write_all(
                Dockerfile {}
                    .render()
                    .expect("Failed to render Dockerfile")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to Dockerfile");

    // Render main.py
    File::create(format!("./{}/main.py", name))
        .and_then(|mut file| file.write_all(MainPy {}.render().expect("Render fail").as_bytes()))
        .expect("Failed to render or write to main.py");

    // Render pre-commit conf
    File::create(format!("./{}/.pre-commit-config.yaml", name))
        .and_then(|mut file| {
            file.write_all(
                PreCommitConfig { python: true }
                    .render()
                    .expect("Failed to render .pre-commit-config.yaml")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to .pre-commit-config.yaml");

    // Render Flake8 conf
    File::create(format!("./{}/.cpa/flake8.cfg", name))
        .and_then(|mut file| {
            file.write_all(
                Flake8 {}
                    .render()
                    .expect("Failed to render flake8.cfg")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to flake8.cfg");

    // Render Prettier conf
    File::create(format!("./{}/.cpa/prettier.json", name))
        .and_then(|mut file| {
            file.write_all(
                Prettier {}
                    .render()
                    .expect("Failed to render prettier.json")
                    .as_bytes(),
            )
        })
        .expect("Failed to create or write to prettier.json");

    // Render Poetry conf
    let re = Regex::new(r"python(3\.\d+|4\.\d+)").unwrap();
    let (python_ver, black_target_ver) = if let Some(caps) = re.captures(&preset) {
        let ver = caps[1].to_string();
        (ver.clone(), format!("py{}", ver.replace('.', "")))
    } else {
        eprintln!("Python version not recognized in --preset, invalid input");
        process::exit(1);
    };

    let pyproj: PyProject = PyProject {
        name: name.clone(),
        python_ver,
        black_target_ver,
    };
    let out_pyproj: String = pyproj.render().expect("Failed to render");
    let mut f_pyproj =
        File::create(format!("./{}/pyproject.toml", name)).expect("Could not create file");
    f_pyproj
        .write_all(out_pyproj.as_bytes())
        .expect("Could not write to file");
}
