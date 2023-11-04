extern crate regex;

use askama::Template;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::process;

#[derive(Template)]
#[template(path = ".pre-commit-config.yaml", escape = "none")]
struct PreCommitConfig {
    python: bool,
}
#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
struct GitIgnore {}

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

pub fn derive_preset(mut preset: String, name: String) {
    if preset == "python" {
        preset = "python3.10".to_string();
    }

    let _ = fs::create_dir_all(format!("./{}/.cpa", name));
    // Render gitignore
    let gi = GitIgnore {};
    let out_gi = gi.render().expect("Failed to render");
    let mut file_gi =
        File::create(format!("./{}/.gitignore", name)).expect("Could not create file");
    file_gi
        .write_all(out_gi.as_bytes())
        .expect("Could not write to file");

    // Render pre-commit conf
    let pc = PreCommitConfig { python: true };
    let out_pc = pc.render().expect("Failed to render");
    let mut file_pc =
        File::create(format!("./{}/.pre-commit-config.yaml", name)).expect("Could not create file");
    file_pc
        .write_all(out_pc.as_bytes())
        .expect("Could not write to file");

    // Render conf for each pre-commit hook
    let f8 = Flake8 {};
    let out_f8 = f8.render().expect("Failed to render");
    let mut f_f8 =
        File::create(format!("./{}/.cpa/flake8.cfg", name)).expect("Could not create file");
    f_f8.write_all(out_f8.as_bytes())
        .expect("Could not write to file");

    let p = Prettier {};
    let out_p = p.render().expect("Failed to render");
    let mut f_p =
        File::create(format!("./{}/.cpa/prettier.json", name)).expect("Could not create file");
    f_p.write_all(out_p.as_bytes())
        .expect("Could not write to file");

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
        File::create(format!("./{}/pypropoject.toml", name)).expect("Could not create file");
    f_pyproj
        .write_all(out_pyproj.as_bytes())
        .expect("Could not write to file");
}
