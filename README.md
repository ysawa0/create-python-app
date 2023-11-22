# CPA: Create-Python-App

![CPA Logo](cpa.png)

`cpa` is a cli tool for ultra fast setup of new Python & Rust projects. It automates the creation of config files like style & lint checks, gitignore, Dockerfile, dependency mangement, etc. An opinionated set of pre-commit hooks are included for enforcing best practices & reducing dev time.

Example outputs are provided in [./example](https://github.com/ysawa0/create-python-app/tree/main/example)

# Installation

#### MacOS, Linux

Install via script below or get from [Releases](https://github.com/ysawa0/create-python-app/releases)

```bash
curl -sSL https://raw.githubusercontent.com/ysawa0/create-python-app/main/install.sh | bash
```

```bash
# cpa will be installed to ~/bin/cpa
# The installer will add ~/bin to your PATH
```

#### Windows

Download latest binary from [Releases](https://github.com/ysawa0/create-python-app/releases)

#### Building from source

```bash
cargo install --path .
```

# Usage

To create a new project:

```bash
cpa create --name myproject
```

Optional params:

- `--preset`: Specifies a preset for the project. Defaults to "python3.10". "pythonx.yz" and "rust" are supported.

Example:

```bash
cpa create --name myproject --preset python3.10
cpa create --name myproject --preset rust
```

Update current working directory with CPA preset.

```bash
cpa update --name myproject --preset python3.10
cpa update --name myproject --preset rust
```

# Goals

- **Speed up Project Creation**: Reduce the time spent on repetitive setup tasks
- **Best Practices**: Encourage best practices for code quality, formatting, and style by including configs for tools like `black`, `isort`, and `flake8`.
- **Automation**: Automate tasks such as generating `.gitignore` files, setting up pre-commit hooks, Github Action CI, configuring linters & formatters.
- Supports Rust and Python.
- Golang support planned

# Contributions and Feedback

Users are welcome to contribute to the project by submitting pull requests or opening issues for bugs and feature requests. Feedback is also greatly appreciated to help improve the tool.
