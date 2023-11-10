# CPA: Create-Python-App

![CPA Logo](cpa.png)

`cpa` is a cli tool for ultra fast setup of new Python projects. It automates the creation of config files for style & lint checks, gitignore, a basic Dockerfile and Poetry for dependency management. An opinionated set of pre-commit hooks are included for enforcing best practices and reducing dev time.

An example output is provided in [./example](https://github.com/ysawa0/create-python-app/tree/main/example)

# Installation

### MacOS, Linux

Install via script below or get it from [Releases](https://github.com/ysawa0/create-python-app/releases)

```bash
curl -sSL https://raw.githubusercontent.com/ysawa0/create-python-app/main/install.sh | bash
```

```bash
# cpa will be installed to ~/bin/cpa
# add ~/bin to your PATH
# eg: echo "export PATH=$PATH:~/bin" >> ~/.zshrc
```

### Windows

Download latest binary from [Releases](https://github.com/ysawa0/create-python-app/releases) page

### Building from source

```bash
# cd to project
cargo install --path .
```

# Usage

To create a new project:

```bash
cpa create --name myproject
```

Optional params:

- `--preset`: Specifies a Python version for the project. Defaults to "python3.10"

Example:

```bash
cpa create --name myproject --preset python3.10
```

Update current working directory with CPA preset.

```bash
cpa update --preset python3.10
```

# Goals

- **Speed up Project Creation**: Reduce the time spent on repetitive setup tasks
- **Best Practices**: Encourage best practices for code quality, formatting, and style by including configs for tools like `black`, `isort`, and `flake8`.
- **Automation**: Automate tasks such as generating `.gitignore` files, setting up pre-commit hooks, and configuring code linters and formatters.
- Golang, Rust support planned

# Contributions and Feedback

Users are welcome to contribute to the project by submitting pull requests or opening issues for bugs and feature requests. Feedback is also greatly appreciated to help improve the tool.
