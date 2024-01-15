
# Default target
all: clean test build deploy

# Clean target
clean:
	@echo "Cleaning..."
	@cargo clean

# Test target
test:
	@echo "Running tests..."
	@cargo test

# Build target
build:
	@echo "Building..."
	@cargo lambda build --release

# Deploy target
deploy:
	@echo "Deploying..."
	@sls deploy

.PHONY: all clean test build deploy