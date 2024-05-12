SHFMT_VERSION := v3.8.0

.PHONY: setuppc
setuppc:
	@echo "Setting up pre-commit and hooks..."
	python3 -m pip install pre-commit
	pre-commit install

ifeq ($(shell uname),Darwin)
	@echo "Setting up shfmt (macOS)..."
	brew install shfmt

	@echo "Setting up shellcheck (macOS)..."
	brew install shellcheck
else
	@echo "Setting up shfmt (Linux)..."
	wget -qO shfmt "https://github.com/mvdan/sh/releases/download/${SHFMT_VERSION}/shfmt_${SHFMT_VERSION}_$(shell uname -m)"
	chmod +x shfmt
	sudo mv shfmt /usr/local/bin/

	@echo "Setting up shellcheck (Linux)..."
	sudo apt-get install shellcheck || sudo yum install shellcheck || sudo dnf install shellcheck
endif

.PHONY: reqtxt
reqtxt:
	poetry export -f requirements.txt --output requirements.txt

