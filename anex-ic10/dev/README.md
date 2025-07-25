# Development Directory

This directory contains development tools and test files for the IC10 Language Server project.

## Directory Structure

```
dev/
├── executables/          # Platform-specific binaries (ignored by git)
│   ├── *.exe            # Windows executables
│   └── *.pdb            # Debug symbols
└── testing/             # Test files and test modules
    ├── *.ic10           # IC10 test scripts
    └── *.rs             # Rust test modules
```

## Building Development Tools

The executables in the `executables/` directory are built from Rust source files located in the `documentation/` directory.

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Cargo (included with Rust)

### Build Instructions

#### 1. Build parse_stationpedia.exe
```bash
# From repository root
cd documentation/
rustc parse_stationpedia.rs -o ../anex-ic10/dev/executables/parse_stationpedia.exe
```

#### 2. Build generate_comprehensive_mappings.exe
```bash
# From repository root
cd documentation/
rustc generate_comprehensive_mappings.rs -o ../anex-ic10/dev/executables/generate_comprehensive_mappings.exe
```

### Alternative: Using Cargo
If you prefer using Cargo, you can create a temporary `Cargo.toml` file:

```bash
# From documentation/ directory
cargo init --name dev_tools
# Move .rs files to src/
# Build with: cargo build --release
```

## Test Files

The `testing/` directory contains comprehensive test files for various language server features:

### IC10 Test Scripts (*.ic10)
- `test_arrow_display.ic10` - Tests inlay hint arrow display
- `test_battery_devices.ic10` - Tests battery device completion and tooltips
- `test_comprehensive_completion.ic10` - Tests instruction and device completions
- `test_fuzzy_search.ic10` - Tests fuzzy search functionality for HASH() completion
- `test_hash_tooltips.ic10` - Tests device hash hover tooltips
- `test_hover.ic10` - Tests hover information for instructions
- And more...

### Rust Test Modules (*.rs)
- `test_hash_lookup.rs` - Unit tests for device hash lookup functionality
- `test_hash_debug.rs` - Debug utilities for hash value verification

## Running Tests

### Manual Testing
1. Open any `.ic10` file in VS Code with the IC10 extension installed
2. Test features like:
   - Code completion (type "HASH(" and see device suggestions)
   - Hover information (hover over instructions or device hashes)
   - Inlay hints (should show arrow symbols with device names)
   - Diagnostics (syntax errors, length limits)

### Automated Testing (Future)
The test files in this directory are designed to support automated testing:

```bash
# Future CI/CD pipeline commands
cargo test                    # Run Rust unit tests
npm test                     # Run VS Code extension tests (from anex-ic10-language-support/)
```

## Development Workflow

1. **Add new test cases**: Create `.ic10` files demonstrating features
2. **Test manually**: Use VS Code to verify language server behavior
3. **Build tools**: Rebuild executables when source files change
4. **Update tests**: Keep test files synchronized with new features

## Contributing

When adding new features:

1. Create corresponding test files in `testing/`
2. Document expected behavior in test comments
3. Test both positive and negative cases
4. Update this README if new tools are added

## Notes

- **Executables are ignored**: Only source files are version controlled
- **Platform-specific**: Built executables work only on the target platform
- **Rebuild required**: Regenerate executables after source changes
- **Test coverage**: Aim for comprehensive test coverage of language server features

---

**Last Updated**: 2025-01-25
**Maintained By**: IC10 Language Server Development Team