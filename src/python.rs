extern crate regex;

use std::{
    fs::{self, File}, io::Write, process
};

use askama::Template;
use regex::Regex;

#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
struct GitIgnore {}

#[derive(Template)]
#[template(path = ".vscode/settings.json", escape = "none")]
struct VSCodeSettings {}

#[derive(Template)]
#[template(path = ".vscode/extensions.json", escape = "none")]
struct VSCodeExtensions {}

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

#[derive(Template)]
#[template(path = ".github/workflows/ci.yaml", escape = "none")]
struct GHWorkflowCI {}

pub fn setup_preset(mut preset: String, name: String, create: bool) {
    if preset == "python" {
        preset = "python3.10".to_string();
    }
    let mut prefix: String = "./".to_string();
    if create {
        prefix = format!("./{}", name)
    }

    // Create needed dirs
    let _ = fs::create_dir_all(format!("{}/.cpa", prefix));
    let _ = fs::create_dir_all(format!("{}/.vscode", prefix));
    let _ = fs::create_dir_all(format!("{}/.github/workflows", prefix));

    // Render Github Actions CI
    File::create(format!("{}/.github/workflows/ci.yaml", prefix))
        .and_then(|mut file| file.write_all(GHWorkflowCI {}.render().expect("Render fail: ci.yaml").as_bytes()))
        .expect("Write fail: ci.yaml");

    // Render .vscode/settings.json
    File::create(format!("{}/.vscode/settings.json", prefix))
        .and_then(|mut file| file.write_all(VSCodeSettings {}.render().expect("Render fail: .vscode/settings.json").as_bytes()))
        .expect("Write fail: .vscode/settings.json");

    // Render .vscode/extensions.json
    File::create(format!("{}/.vscode/extensions.json", prefix))
        .and_then(|mut file| {
            file.write_all(
                VSCodeExtensions {}
                    .render()
                    .expect("Render fail: .vscode/extensions.json")
                    .as_bytes(),
            )
        })
        .expect("Write fail: .vscode/extensions.json");

    // Render .gitignore
    File::create(format!("{}/.gitignore", prefix))
        .and_then(|mut file| file.write_all(GitIgnore {}.render().expect("Render fail: .gitignore").as_bytes()))
        .expect("Write fail: .gitignore");

    // Render Makefile
    File::create(format!("{}/Makefile", prefix))
        .and_then(|mut file| file.write_all(Makefile {}.render().expect("Render fail: Makefile").as_bytes()))
        .expect("Write fail: Makefile");

    // Render Dockerfile
    File::create(format!("{}/Dockerfile", prefix))
        .and_then(|mut file| file.write_all(Dockerfile {}.render().expect("Render fail: Dockerfile").as_bytes()))
        .expect("Write fail: Dockerfile");

    // Render main.py
    File::create(format!("{}/main.py", prefix))
        .and_then(|mut file| file.write_all(MainPy {}.render().expect("Render fail").as_bytes()))
        .expect("Render fail: main.py");

    // Render pre-commit conf
    File::create(format!("{}/.pre-commit-config.yaml", prefix))
        .and_then(|mut file| {
            file.write_all(
                PreCommitConfig { python: true }
                    .render()
                    .expect("Render fail: .pre-commit-config.yaml")
                    .as_bytes(),
            )
        })
        .expect("Write fail: .pre-commit-config.yaml");

    // Render Flake8 conf
    File::create(format!("{}/.cpa/flake8.cfg", prefix))
        .and_then(|mut file| file.write_all(Flake8 {}.render().expect("Render fail: flake8.cfg").as_bytes()))
        .expect("Write fail: flake8.cfg");

    // Render Prettier conf
    File::create(format!("{}/.cpa/prettier.json", prefix))
        .and_then(|mut file| file.write_all(Prettier {}.render().expect("Render fail: prettier.json").as_bytes()))
        .expect("Write fail: prettier.json");

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
    let mut f_pyproj = File::create(format!("{}/pyproject.toml", prefix)).expect("Could not create file");
    f_pyproj.write_all(out_pyproj.as_bytes()).expect("Could not write to file");
    println!("Project created at: {}", prefix)
}
