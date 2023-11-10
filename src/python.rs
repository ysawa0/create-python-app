extern crate regex;

use std::{
    fs::{self, File}, io::Write, process
};

use askama::Template;
use regex::Regex;

trait CPATemplate {
    fn write(&self, prefix: &str, path: &str);
}

// Implement the new trait for any type that implements `askama::Template`
impl<T: Template> CPATemplate for T {
    fn write(&self, prefix: &str, path: &str) {
        let content = append_eof(self.render().expect("Failed to render file."));
        let mut f = File::create(format!("{}/{}", prefix, path)).expect("Could not create file");
        f.write_all(content.as_bytes()).expect("Could not write to file");
    }
}

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

fn append_eof(mut s: String) -> String {
    s.push('\n');
    s
}

pub fn setup_preset(mut preset: &str, name: String, create: bool) {
    if preset == "python" {
        preset = "python3.10";
    }
    let mut prefix: String = "./".to_string();
    if create {
        prefix = format!("./{}", name)
    }

    // Create needed dirs
    let _ = fs::create_dir_all(format!("{}/.cpa", prefix));
    let _ = fs::create_dir_all(format!("{}/.vscode", prefix));
    let _ = fs::create_dir_all(format!("{}/.github/workflows", prefix));

    // Render files
    GHWorkflowCI {}.write(&prefix, ".github/workflows/ci.yaml");
    VSCodeSettings {}.write(&prefix, ".vscode/settings.json");
    VSCodeExtensions {}.write(&prefix, ".vscode/extensions.json");
    GitIgnore {}.write(&prefix, ".gitignore");
    Makefile {}.write(&prefix, "Makefile");
    Dockerfile {}.write(&prefix, "Dockerfile");
    MainPy {}.write(&prefix, "main.py");
    PreCommitConfig { python: true }.write(&prefix, ".pre-commit-config.yaml");
    Flake8 {}.write(&prefix, ".cpa/flake8.cfg");
    Prettier {}.write(&prefix, ".cpa/prettier.json");

    // Render Poetry conf
    let re = Regex::new(r"python(3\.\d+|4\.\d+)").unwrap();
    let (python_ver, black_target_ver) = if let Some(caps) = re.captures(preset) {
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
    pyproj.write(&prefix, "pyproject.toml");
    println!("Project created at: {}", prefix)
}
