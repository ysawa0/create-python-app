# CPA: Create-Python-App

## Overview

`cpa` is a cli tool designed to streamline the setup of new Python projects. It automates the creation of Python projects with commonly used configs and boilerplate.

## Goals

- **Speed up Project Creation**: Reduce the time spent on repetitive setup tasks
- **Best Practices**: Encourage best practices for code quality, formatting, and style by including configs for tools like `black`, `isort`, and `flake8`.
- **Automation**: Automate tasks such as generating `.gitignore` files, setting up pre-commit hooks, and configuring code linters and formatters.

## Features

- Provides pre-commit hook setup with hooks for checking merge conflicts, large files, and code styling.

## Installation

Download binary from Github

```bash

```

Building from source

```bash
# cd to project
cargo install --path .
```

## Usage

To create a new project:

```bash
cpa create --name <project_name>
```

Optional params:

- `--preset`: Specifies a Python version for the project. Defaults to "python" which is mapped internally to "python3.10".

Example:

```bash
cpa create --name my_project --preset python3.10
```

## Contributions and Feedback

Users are welcome to contribute to the project by submitting pull requests or opening issues for bugs and feature requests. Feedback is also greatly appreciated to help improve the tool.
