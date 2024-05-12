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
ifeq ($(shell uname),x86_64)
	@echo "Setting up shfmt (Linux)..."
	SHFMT_BIN="shfmt_${SHFMT_VERSION}_linux_amd64"
	wget -qO shfmt "https://github.com/mvdan/sh/releases/download/${SHFMT_VERSION}/${SHFMT_BIN}"
	chmod +x shfmt
	sudo mv shfmt /usr/local/bin/shfmt
	@echo "Setting up shellcheck (Linux)..."
	sudo apt-get install shellcheck || sudo yum install shellcheck || sudo dnf install shellcheck
else
	@echo "Architecture not supported! Update this Makefile!"
	exit 1
endif

.PHONY: reqtxt
reqtxt:
	poetry export -f requirements.txt --output requirements.txt

.PHONY: pcao
pcao:
	pre-commit autoupdate
	pre-commit autoupdate -c templates/base/.pre-commit-config.yaml
