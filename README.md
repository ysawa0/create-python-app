# CPA: Create-Python-App

![CPA Logo](cpa.png)

## Overview

`cpa` is a cli tool for ultra fast setup of new Python projects. It automates the creation of config files for style & lint checks, gitignore, a basic Dockerfile and dependency management configuration. An opinionated set of pre-commit hooks are included for enforcing best practices and reducing setup time.

## Installation

### MacOS, Linux

Download latest binary and install via provided `install.sh` script or get it from [Releases](https://github.com/ysawa0/create-python-app/releases)

```
sh install.sh
```

### Windows

Download latest binary from [Releases](https://github.com/ysawa0/create-python-app/releases) page

### Building from source

```bash
# cd to project
cargo install --path .
```

## Usage

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

## Goals

- **Speed up Project Creation**: Reduce the time spent on repetitive setup tasks
- **Best Practices**: Encourage best practices for code quality, formatting, and style by including configs for tools like `black`, `isort`, and `flake8`.
- **Automation**: Automate tasks such as generating `.gitignore` files, setting up pre-commit hooks, and configuring code linters and formatters.
- Golang, Rust support planned

## Contributions and Feedback

Users are welcome to contribute to the project by submitting pull requests or opening issues for bugs and feature requests. Feedback is also greatly appreciated to help improve the tool.
