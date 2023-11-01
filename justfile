# List all recipes
default:
	just --list --unsorted

# Build all the applications
build-all-apps:
	cargo build --release --locked
