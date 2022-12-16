.phony:

env:
	export $(cat .env | xargs)

watch:
		@echo "Watching for changes..."
		cargo watch -s 'mold -run cargo run'

run:
		@echo "Running..."
		cargo run

build:
		@echo "Building..."
		cargo build

test:
		@echo "Testing..."
		cargo test

clean:
		@echo "Cleaning..."
		cargo clean

