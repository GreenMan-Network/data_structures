SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

version:	## Show Rust versions
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

test:			## Run tests
	cargo test --quiet

run:			## Run the application
	cargo run

build: ## Build the project using cargo
	cargo build

release:		## Build the application in release mode
	cargo build --release

all:			## Run all tasks: format, lint, test, run
	format lint test run

clean:	## Clean the project using cargo
	cargo clean

document: ## Generate the documentation using cargo
	cargo doc

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version of the project using cargo
	@echo "Current version: $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter the new version: " version; \
	updated_version=$$(echo $(shell cargo pkgid | cut -d# -f2) | sed -E "s/[0-9]+\.[0-9]+\.[0-9]+/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"