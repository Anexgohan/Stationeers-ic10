# Changelog

All notable changes to this project will be documented in this file.

## [1.1.2] - 2025-01-25

### 🐛 Critical Bug Fixes
- **Fixed incorrect device hash values** - Corrected 4 device hashes that were causing wrong tooltips and hints
  - `StructureBattery`: Fixed from 700133157 to -400115994 (Station Battery)
  - `StructureBatteryLarge`: Fixed from -459827268 to -1388288459 (Station Battery Large)
- **Added missing transformer devices** - Added 2 missing transformer variants to device registry
  - `StructureTransformerSmall`: Added hash -890946730 (Transformer Small)
  - `StructureTransformerMedium`: Added hash -1065725831 (Transformer Medium)
- **Prevented Kit vs Structure hash confusion** - Systematic validation against authoritative stationpedia.txt
- **Updated documentation** - Added hash validation process and correction history

### 🔧 Improvements
- **Hash Validation Process** - Established systematic verification against authoritative source data
- **Documentation Updates** - Updated task_hash-tooltip.md and hashes_ids.md with correct values

## [1.1.1] - 2025-07-25

### 📄 Documentation
- **Added README.md to extension package** - Fixes "No README available" issue in VS Code
- Enhanced extension documentation with comprehensive feature descriptions
- Added usage examples and configuration guide

## [1.1.0] - 2025-07-25

### ✨ New Features
- **HASH() Function Support**: Inline hints and tooltips for device hash calculations
- **100+ Device Mappings**: Complete database of Stationeers devices with hash values
- **Smart Typo Handling**: Fuzzy matching for device names (e.g., "StructurePipeAnalysizer" works)
- **Enhanced Code Completion**: Intelligent suggestions for all IC10 instructions
- **4096-Byte Limit Validation**: Real-time warnings when approaching Stationeers' code size limits
- **Hover Tooltips**: Show friendly device names when hovering over hash values
- **Go-to-Definition**: Jump to label and variable definitions
- **Comprehensive Diagnostics**: Advanced syntax error detection and validation

### 🔧 Improvements
- **Language Server Performance**: Significantly improved response times and stability
- **Instruction Database**: Updated with latest Stationeers instruction set
- **Error Messages**: More descriptive and helpful diagnostic messages
- **Code Validation**: Line length and column limit checking
- **Configuration Options**: Customizable limits for lines, columns, and bytes

### 🐛 Bug Fixes
- Fixed syntax highlighting edge cases
- Resolved memory leaks in language server
- Corrected instruction parameter validation
- Fixed completion provider conflicts
- Improved error recovery in parser

### 📁 Project Structure
- Reorganized documentation into dedicated directory
- Moved testing files to organized development structure
- Updated build and packaging workflows
- Enhanced README with comprehensive usage guide

## [1.0.1](https://github.com/Anexgohan/Stationeers-ic10/compare/v1.0.0...v1.0.1) (2025-07-12)

### Features

* **build:** Automated release process with GitHub Actions.
* **docs:** Updated `instructions-for-compile.md` with relative paths.
* **docs:** Added "Special Thanks" section to `README.md`.

## [1.0.0](https://github.com/Anexgohan/Stationeers-ic10/compare/v0.4.0...v1.0.0) (2025-07-12)

### Features

* **linter:** Added 4096-byte size limit check.
* **linter:** Removed incorrect warning for numeric batch modes.
* **extension:** Changed author to "Anex".
* **extension:** Default column limit to 90.
* **build:** Created a self-contained project.
* **build:** Updated build process to compile linter from source.
* **repo:** Forked from https://github.com/awilliamson/ic10-language-support and https://github.com/Xandaros/ic10lsp