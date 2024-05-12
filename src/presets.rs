extern crate regex;
use std::{
    fs::{self, File}, io::Write
};

use askama::Template;

use crate::Language;

trait CPATemplate {
    fn write(&self, prefix: &str, path: &str);
}

// Implement convenience trait for any type that implements `askama::Template`
impl<T: Template> CPATemplate for T {
    fn write(&self, prefix: &str, path: &str) {
        let mut content = self.render().expect("Failed to render file.");
        content.push('\n');
        let mut f = File::create(format!("{}/{}", prefix, path)).expect("Could not create file");
        f.write_all(content.as_bytes()).expect("Could not write to file");
    }
}

////////////////////////////////////
// COMMON
////////////////////////////////////
#[derive(Template)]
#[template(path = ".gitignore", escape = "none")]
pub struct GitIgnore {}

#[derive(Template)]
#[template(path = ".vscode/settings.json", escape = "none")]
pub struct VSCodeSettings {}

#[derive(Template)]
#[template(path = ".vscode/extensions.json", escape = "none")]
pub struct VSCodeExtensions {}

#[derive(Template)]
#[template(path = "Makefile", escape = "none")]
pub struct Makefile {}

#[derive(Template)]
#[template(path = ".github/workflows/ci.yaml", escape = "none")]
pub struct GhCI {}

#[derive(Template)]
#[template(path = "base/ci.yaml", escape = "none")]
pub struct GhCIBase {}

#[derive(Template)]
#[template(path = ".ci/prettier.json", escape = "none")]
pub struct Prettier {}

#[derive(Template)]
#[template(path = ".pre-commit-config.yaml", escape = "none")]
pub struct PreCommitConfig {
    pub language: String,
}

#[derive(Template)]
#[template(path = "base/.pre-commit-config.yaml", escape = "none")]
pub struct PreCommitConfigBase {
    pub language: String,
}

////////////////////////////////////
// PYTHON
////////////////////////////////////
#[derive(Template)]
#[template(path = "python/Dockerfile", escape = "none")]
pub struct PyDockerfile {}

#[derive(Template)]
#[template(path = "python/pyproject.toml", escape = "none")]
pub struct PyProject {
    pub name: String,
    pub python_ver: String,
    pub black_target_ver: String,
}

#[derive(Template)]
#[template(path = ".ci/flake8.cfg", escape = "none")]
pub struct Flake8 {}

////////////////////////////////////
// RUST
////////////////////////////////////
#[derive(Template)]
#[template(path = "rust/Cargo.toml", escape = "none")]
struct CargoToml {
    name: String,
}

#[derive(Template)]
#[template(path = "rust/rustfmt.toml", escape = "none")]
struct RustFmt {}

#[derive(Template)]
#[template(path = "rust/src/main.rs", escape = "none")]
struct RustMain {}

pub fn common(name: &str, create: bool, lang: &Language) -> String {
    let prefix: String = if create { format!("./{}", name) } else { "./".to_string() };

    // Create needed dirs
    let _ = fs::create_dir_all(format!("{}/.ci", prefix));
    let _ = fs::create_dir_all(format!("{}/.vscode", prefix));
    let _ = fs::create_dir_all(format!("{}/.github/workflows", prefix));

    // Render common files
    GhCI {}.write(&prefix, ".github/workflows/ci.yaml");
    GitIgnore {}.write(&prefix, ".gitignore");
    Makefile {}.write(&prefix, "Makefile");
    PreCommitConfig {
        language: lang.language.to_string(),
    }
    .write(&prefix, ".pre-commit-config.yaml");
    Prettier {}.write(&prefix, ".ci/prettier.json");
    VSCodeSettings {}.write(&prefix, ".vscode/settings.json");
    VSCodeExtensions {}.write(&prefix, ".vscode/extensions.json");
    prefix
}

pub fn python(name: &str, prefix: &str, lang: &Language) {
    let black_target_ver = format!("py{}", lang.ver.replace('.', ""));

    // Render Python-specific files
    Flake8 {}.write(prefix, ".ci/flake8.cfg");
    PyDockerfile {}.write(prefix, "Dockerfile");

    let pyproj: PyProject = PyProject {
        name: name.to_string(),
        python_ver: lang.ver.to_string(),
        black_target_ver,
    };
    pyproj.write(prefix, "pyproject.toml");
}

pub fn rust(name: &str, prefix: &str) {
    let _ = fs::create_dir_all(format!("{}/src", prefix));

    // Render Python-specific files
    RustMain {}.write(prefix, "src/main.rs");
    CargoToml { name: name.to_string() }.write(prefix, "Cargo.toml");
    RustFmt {}.write(prefix, "rustfmt.toml");
}

pub fn base(name: &str, create: bool, lang: &Language) -> String {
    let prefix: String = if create { format!("./{}", name) } else { "./".to_string() };

    // Create needed dirs
    let _ = fs::create_dir_all(format!("{}/.ci", prefix));
    let _ = fs::create_dir_all(format!("{}/.github/workflows", prefix));

    // Render common files
    GhCIBase {}.write(&prefix, ".github/workflows/ci.yaml");
    GitIgnore {}.write(&prefix, ".gitignore");
    Makefile {}.write(&prefix, "Makefile");
    PreCommitConfigBase {
        language: lang.language.to_string(),
    }
    .write(&prefix, ".pre-commit-config.yaml");
    Prettier {}.write(&prefix, ".ci/prettier.json");
    prefix
}
