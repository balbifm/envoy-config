.PHONY: build clean run

# Compile the filter
build:
	@echo "> Compiling the filter as a Transform body WASM module...."
	mkdir -p build
	cargo build --lib --target=wasm32-unknown-unknown --release
	@echo "> Transform body policy successfully compiled"


# Deleting the build artifacts.
clean:
	rm -rf target

# Running the setup using docker-compose.
run:
	@echo >&2 "************************************************************************"
	@echo >&2 " Before running the setup, build the filter and service using make build"
	@echo >&2 "************************************************************************"
	cd test && docker compose up
