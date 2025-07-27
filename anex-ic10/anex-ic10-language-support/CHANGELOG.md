# Changelog

All notable changes to this project will be documented in this file.

## [1.6.0] - 2025-01-27

### 🐛 Critical Bug Fixes
- **Complete Instruction Documentation Coverage** - Added missing documentation for 6 IC10 instructions
  - **lbn** - Loads var from all output network devices with provided type hash using batch mode
  - **sbn** - Stores register value to var on all output network devices with provided type hash
  - **lbs** - Loads slot var from devices using batch mode with provided slot
  - **lbns** - Loads slot var from devices with type filtering using batch mode
  - **not** - Logical NOT operation (r0 = 1 if input is 0, else 0)
  - **sla/sll/sra/srl** - Complete bit shift operations (arithmetic and logical, left and right)
- **Syntax Highlighting Fix** - Batch instructions now display proper coloring in hover tooltips
  - **TextMate Grammar Update** - Added missing batch instruction patterns (lbn, sbn, lbs, sbs, lbns, sbns)
  - **Hover Content Structure** - Fixed syntax highlighting by using separate LanguageString elements for examples
- **Documentation Integrity** - All language server tests now pass, ensuring complete instruction coverage

### 🔧 Improvements
- **Enhanced Examples Coverage** - Added comprehensive examples for all previously undocumented instructions
  - **Batch Operations** - Real-world examples for slot-based and network batch operations
  - **Bit Operations** - Practical shift operation examples with mathematical use cases
  - **Beginner-Friendly Patterns** - Simple r0-r15 examples for all new instructions
- **Comprehensive Categorization** - Added proper categorization for new instructions
  - **Batch Operations** - lbs, lbns properly categorized with other batch instructions
  - **Logic Operations** - not instruction added to logic category
  - **Bit Operations** - New category for shift operations (sla, sll, sra, srl)
- **Related Instructions Mapping** - Complete cross-reference system for all instructions
  - **130+ Instruction Relationships** - Enhanced related instruction discovery
  - **Category-Based Grouping** - Logical groupings for better instruction exploration

### 🎨 User Experience Improvements
- **Simplified Interface Design** - Streamlined hover tooltips based on user feedback
  - **Removed Complex Code Actions** - Eliminated overwhelming interactive features per user request
  - **Clean Documentation Focus** - Prioritized core documentation over feature complexity
  - **Optional Guidance** - Minimal Code Actions hint for advanced users who want VS Code integration
- **Professional VS Code Integration** - Maintains VS Code standards without feature bloat
  - **Lightweight Tooltips** - Fast, focused documentation without unnecessary interactions
  - **User-Tested Design** - Interface refined through actual usage and feedback iteration

### 📊 Technical Achievements
- **100% Instruction Coverage** - Every IC10 instruction now has complete documentation
- **Test Suite Validation** - All language server tests pass with comprehensive coverage
- **Performance Optimization** - Efficient PHF map lookups for instant tooltip generation
- **Data Accuracy** - Documentation verified against official game data (english.xml, stationpedia.txt)

### 🔧 Code Structure Improvements
- **New Module: tooltip_documentation.rs** - Dedicated Rust module for enhanced hover documentation
  - **318 Lines of Documentation Code** - Comprehensive instruction examples and categorization system
  - **INSTRUCTION_EXAMPLES Map** - 60+ instruction examples with real-world automation patterns
  - **INSTRUCTION_CATEGORIES Map** - 8 functional categories for organized instruction discovery
  - **RELATED_INSTRUCTIONS Map** - 150+ cross-reference mappings for instruction exploration
- **Enhanced Language Server Architecture** - Improved hover provider with rich markdown content
  - **Syntax-Highlighted Examples** - Each example rendered as proper IC10 code in hover tooltips
  - **LanguageString Integration** - Separate syntax highlighting for each code example
  - **Markdown Formatting** - Professional documentation layout with categories and related instructions
- **VS Code Extension Enhancements** - Command registration and palette integration
  - **Command Handlers** - Full implementation of ic10.showRelated, ic10.searchCategory, ic10.showExamples
  - **Quick Pick Integration** - Professional VS Code selection interfaces for instruction exploration
  - **Package.json Updates** - Proper command declarations and metadata for marketplace compatibility

## [1.5.0] - 2025-01-27

### ✨ New Features
- **Enhanced Hover Documentation** - Comprehensive instruction tooltips with examples and interactive features
  - **Multi-Example Coverage** - 3+ examples per instruction with learning progression (simple → intermediate → advanced)
  - **Simplified Register Examples** - Every instruction includes beginner-friendly examples using r0-r15 and d0-d5
  - **Real-World Patterns** - Examples derived from actual IC10 automation scripts for authentic usage
  - **Interactive Command Links** - Clickable links in hover tooltips for related instructions and categories
  - **Comprehensive Categorization** - 8 instruction categories (Arithmetic, Device I/O, Batch Operations, etc.)
  - **Related Instruction Discovery** - Quick access to similar commands through enhanced tooltips
- **Interactive VS Code Commands** - Two new commands for instruction exploration
  - **Show Related Instructions** - `ic10.showRelated` command displays related instructions in Quick Pick interface
  - **Search by Category** - `ic10.searchCategory` command browses instructions by functional categories
  - **Smart Insertion** - Both commands support cursor-based insertion of selected instructions

### 🔧 Improvements
- **tooltip_documentation.rs Module** - Dedicated Rust module with PHF maps for efficient documentation lookup
  - **57 Instruction Examples** - Comprehensive coverage with authentic automation patterns
  - **140+ Related Mappings** - Cross-reference system linking similar instructions
  - **8 Category Classifications** - Organized instruction groupings for better discoverability
- **Enhanced Hover Provider** - Complete overhaul of instruction hover functionality
  - **Rich Markdown Content** - Syntax highlighting, examples, and interactive links
  - **Performance Optimized** - Efficient PHF map lookups for instant tooltip generation
- **VS Code Extension Integration** - Seamless command palette and Quick Pick integration
- **Data Verification** - Examples validated against actual Stationeers game data and real IC10 scripts

### 🐛 Bug Fixes
- **Instruction Examples Accuracy** - Fixed critical command mixing issues in tooltip documentation
  - **lb Command Examples** - Corrected examples that incorrectly used `lbn` commands
  - **sb Command Examples** - Fixed examples that incorrectly used `sbn` commands
  - **bgt Command Examples** - Fixed example that incorrectly showed `bgtz` command
  - **Syntax Consistency** - All 57 instruction examples now properly demonstrate their respective commands

## [1.4.0] - 2025-01-27

### ✨ New Features
- **Register Operation History** - Enhanced hover tooltips with comprehensive register tracking
  - **Operation Timeline** - Shows chronological history of all operations performed on each register
  - **Scrollable History** - Displays up to 99 operations in scrollable VS Code tooltips
  - **Line Number Tracking** - Each operation shows the exact line where it occurred
  - **Full Alias Integration** - Works seamlessly with aliased registers (`temp (r0)`)
  - **Assignment Operation Coverage** - Tracks 40+ IC10 instructions that modify register values

### 🔧 Improvements
- **Simplified Architecture** - Removed complex value computation for better maintainability
- **Enhanced Hover Provider** - Clean, readable tooltips with markdown formatting
- **Scrollable Interface** - Takes advantage of VS Code's native tooltip scrolling capabilities
- **Performance Optimization** - Lightweight operation tracking without heavy computation overhead

### 🐛 Bug Fixes
- **Tooltip Formatting** - Fixed cluttered single-line display with proper multi-line markdown structure
- **Duplicate Detection** - Eliminated repeated entries in operation history
- **Header Duplication** - Removed redundant register name display in hover tooltips

## [1.3.0] - 2025-07-26

### ✨ New Features
- **Register Usage Analysis** - Comprehensive static analysis for IC10 register optimization
  - **Unused Register Detection** - Identifies registers that are declared but never used
  - **Assigned-but-Never-Read Warnings** - Highlights registers that receive values but are never consumed
  - **Read-Before-Assign Errors** - Catches potentially uninitialized register usage
  - **Full Alias Support** - Tracks register usage through aliases (`alias temp r0`)
  - **40+ Instruction Support** - Recognizes all assignment operations including batch loads (`lbn`, `lbs`, `lbns`)
  - **JAL/RA Function Support** - Proper handling of function calls and return addresses
  - **Smart Diagnostics** - Context-aware error messages with register names and aliases

### 🔧 Improvements
- **Language Server Architecture** - Added modular `additional_features.rs` for advanced analysis features
- **AST Analysis Enhancement** - Improved tree-sitter parsing for comprehensive register tracking
- **Diagnostic Integration** - Seamless integration with VS Code Problems panel
- **Real-world Testing** - Validated with complex IC10 scripts (furnace control, airlock systems)

### 🐛 Bug Fixes
- **Batch Instruction Recognition** - Fixed `lbn`, `lbs`, `lbns` detection as assignment operations
- **Alias Resolution Logic** - Enhanced mapping between aliases and register usage
- **Line Ordering Analysis** - Improved read-before-assign detection accuracy
- **Self-Reference Handling** - Proper support for operations like `mul r0 r0 2`

## [1.2.0] - 2025-07-26

### ✨ New Features
- **StationeersDataExtractor BepInEx Plugin** - Automated device hash extraction tool for game data
- **Comprehensive Development Tools** - Added utility programs for hash mapping generation and stationpedia parsing
- **Enhanced Hash Utilities** - Improved hash calculation and lookup functions in language server

### 🔧 Improvements
- **Build Configuration Updates** - Enhanced build process and configuration management
- **Development Workflow** - Added comprehensive toolset for device data extraction and processing
- **Documentation Structure** - Improved project organization and development file structure

### 🐛 Bug Fixes
- **Removed Obsolete Files** - Cleaned up outdated stationpedia.txt file
- **Hash Utility Enhancements** - Fixed hash computation and device lookup functionality

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