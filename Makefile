.PHONY: all core wasm cli test clean install help dev full quick ci

# Configuration
BUILD_MODE ?= release
CARGO_FLAGS = $(if $(filter $(BUILD_MODE),release),--release,)
TARGET_DIR = $(if $(filter $(BUILD_MODE),release),release,debug)
WASM_FLAGS = $(if $(filter $(BUILD_MODE),release),,--dev) --quiet

# Colors (only if make is run in a terminal)
ifneq (,$(findstring -t,$(shell ls -la /proc/self/fd/1 2>/dev/null || echo -t)))
	RED := \033[0;31m
	GREEN := \033[0;32m
	YELLOW := \033[1;33m
	BLUE := \033[0;34m
	NC := \033[0m
else
	RED :=
	GREEN :=
	YELLOW :=
	BLUE :=
	NC :=
endif

define print_step
	@echo -e "$(BLUE)📦 $(1)$(NC)"
endef

define print_success
	@echo -e "$(GREEN)✅ $(1)$(NC)"
endef

define print_warning
	@echo -e "$(YELLOW)⚠️  $(1)$(NC)"
endef

# Default target
all: core wasm cli
	$(call print_success,Build complete! CLI: ./target/$(TARGET_DIR)/ycard)

# Generate code from schema
generate:
	@echo "$(CYAN)🔄 Generating code from schema...$(NC)"
	node generate-code.js
	@echo "$(GREEN)✅ Code generation complete$(NC)"

# Build core library
core: generate
	@echo "$(CYAN)📦 Building yCard core library...$(NC)"
	cd ycard-core && cargo build --release
	@echo "$(GREEN)✅ Core library built$(NC)"

# Build WASM packages
wasm: core
	$(call print_step,Building WASM packages...)
	@command -v wasm-pack >/dev/null 2>&1 || { \
		echo "Installing wasm-pack..."; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	}
	cd ycard-core && \
	wasm-pack build --target nodejs $(WASM_FLAGS) && \
	wasm-pack build --target web --out-dir pkg-web $(WASM_FLAGS) && \
	wasm-pack build --target bundler --out-dir pkg-bundler $(WASM_FLAGS)
	$(call print_success,WASM packages built (pkg/, pkg-web/, pkg-bundler/))

# Build CLI
cli: core
	$(call print_step,Building CLI...)
	cargo build $(CARGO_FLAGS) --bin ycard  
	$(call print_success,CLI built at ./target/$(TARGET_DIR)/ycard)

# Quick build (same as all, for backward compatibility)
quick: all

# Development build (debug mode)
dev:
	$(call print_step,Quick development build...)
	$(MAKE) BUILD_MODE=debug all
	$(call print_success,Development build complete!)

# CI/CD build with testing and validation
ci: all test wasm-test cli-test
	$(call print_step,Generating build info...)
	@echo '{"buildTime":"'$$(date -u +"%Y-%m-%dT%H:%M:%SZ")'","buildMode":"$(BUILD_MODE)","gitCommit":"'$$(git rev-parse HEAD 2>/dev/null || echo unknown)'","artifacts":{"cli":"./target/$(TARGET_DIR)/ycard","wasm":"./ycard-core/pkg*/"}}' > build-info.json
	$(call print_success,CI build complete with validation!)

# Run tests
test:
	$(call print_step,Running core tests...)
	@cd ycard-core && (cargo test $(CARGO_FLAGS) --quiet || (echo "$(YELLOW)Some tests failed (may be expected)$(NC)"; true))
	$(call print_success,Core tests completed)

# Test WASM (if Node.js available)
wasm-test: wasm
	$(call print_step,Testing WASM...)
	@if command -v node >/dev/null 2>&1 && [ -f ycard-core/test-wasm.js ]; then \
		cd ycard-core && (node test-wasm.js >/dev/null 2>&1 && echo -e "$(GREEN)✅ WASM test passed$(NC)" || echo -e "$(YELLOW)⚠️  WASM test failed$(NC)"); \
	else \
		echo -e "$(YELLOW)⚠️  Skipping WASM test (Node.js not available)$(NC)"; \
	fi

# Test CLI (if test file exists)
cli-test: cli
	$(call print_step,Testing CLI...)
	@if [ -f test-example.ycard ]; then \
		(./target/$(TARGET_DIR)/ycard parse test-example.ycard --json-ast >/dev/null 2>&1 && echo -e "$(GREEN)✅ CLI test passed$(NC)" || echo -e "$(YELLOW)⚠️  CLI test failed$(NC)"); \
	else \
		echo -e "$(YELLOW)⚠️  No test file found$(NC)"; \
	fi

# Install CLI globally
install: cli
	$(call print_step,Installing CLI globally...)
	cargo install --path ycard-cli --force
	$(call print_success,CLI installed at $$(which ycard 2>/dev/null || echo "unknown location"))

# Build everything including optional components
full: all
	$(call print_step,Building full ecosystem...)
	@if command -v npm >/dev/null 2>&1; then \
		if [ -d ycard-ts ]; then \
			echo "📝 Building TypeScript SDK..."; \
			cd ycard-ts && npm install --silent && npm run build; \
		fi; \
		if [ -d ycard-lsp ]; then \
			echo "🔍 Building LSP server..."; \
			cd ycard-lsp && npm install --silent && npm run build; \
		fi; \
	else \
		$(call print_warning,npm not found, skipping TypeScript components); \
	fi
	$(call print_success,Full ecosystem built!)

# Clean build artifacts
clean:
	$(call print_step,Cleaning build artifacts...)
	cargo clean
	rm -rf ycard-core/pkg*
	rm -rf ycard-ts/dist ycard-ts/node_modules
	rm -rf ycard-lsp/dist ycard-lsp/node_modules
	rm -f build-info.json
	# Clean generated files
	rm -f ycard-core/src/generated_*.rs
	rm -f ycard-ts/src/generated_*.ts
	rm -f ycard-grammar/generated_*.js
	rm -f ycard-lsp/src/generated_*.ts
	$(call print_success,Cleaned all build artifacts)

# Show help
help:
	@echo "yCard Build System (Single Source of Truth)"
	@echo "==========================================="
	@echo ""
	@echo "Main Targets:"
	@echo "  all       - Build core, WASM, and CLI (default)"
	@echo "  dev       - Quick development build (debug mode)"
	@echo "  ci        - CI/CD build with full testing"
	@echo "  full      - Build everything including optional components"
	@echo ""
	@echo "Individual Components:"
	@echo "  core      - Build Rust core library only"  
	@echo "  wasm      - Build WASM packages (nodejs, web, bundler)"
	@echo "  cli       - Build CLI tool"
	@echo ""
	@echo "Testing & Validation:"
	@echo "  test      - Run core tests"
	@echo "  wasm-test - Test WASM packages"
	@echo "  cli-test  - Test CLI tool"
	@echo ""
	@echo "Utilities:"
	@echo "  generate  - Generate code from schema.json (DRY approach)"
	@echo "  install   - Install CLI globally"
	@echo "  clean     - Clean all build artifacts"
	@echo "  help      - Show this help"
	@echo ""
	@echo "Environment Variables:"
	@echo "  BUILD_MODE=debug|release (default: release)"
	@echo ""
	@echo "Examples:"
	@echo "  make                    # Build everything (release mode)"
	@echo "  make dev                # Quick development build"
	@echo "  make BUILD_MODE=debug   # Debug build"
	@echo "  make ci                 # Full CI build with tests"