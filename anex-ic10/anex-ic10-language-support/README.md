# IC10 Language Support for Stationeers

This Visual Studio Code extension provides syntax highlighting, IntelliSense, and other language support features for the IC10 MIPS-like language used in the game Stationeers.

## Features

- **Syntax highlighting** for IC10 language files (`.ic10`)
- **Code autocompletion** with intelligent suggestions for all IC10 instructions
- **Hover information** with detailed device descriptions and instruction help
- **Signature help** for function parameters and instruction usage
- **Go to definition** for labels and variables
- **Diagnostics** with comprehensive syntax error detection and code length limits
- **🆕 HASH() Function Support** - Advanced device hash calculations with inline hints
  - `define Pump HASH("StructureVolumePump")` → Shows "Volume Pump" inline hint
  - `define Sensor -1252983604` → Shows "Gas Sensor" inline hint  
  - **100+ devices supported** including all common IC10 automation devices
  - **Smart typo handling** (e.g., "StructurePipeAnalysizer" works correctly)
  - **Hover tooltips** show device names for hash values in your code
- **🆕 Enhanced Language Server** with improved performance and stability
- **🆕 Code length validation** - Warns when approaching Stationeers' 4096-byte limit
- **🆕 Comprehensive instruction database** with latest Stationeers updates

## Usage

After installing the extension, open any `.ic10` file in VS Code. The extension will automatically activate and provide language features.

**Key Features:**
- **Smart Completions**: Type instruction names to see all available options with documentation
- **HASH() Tooltips**: Hover over device hashes to see friendly device names
- **Error Detection**: Real-time syntax checking and 4096-byte limit warnings
- **Quick Navigation**: Go-to-definition for labels and variables

## Configuration

The extension supports several VS Code settings:
- `ic10.lsp.max_lines`: Maximum lines allowed (default: 128)
- `ic10.lsp.max_columns`: Maximum columns per line (default: 90)  
- `ic10.lsp.max_bytes`: Maximum total bytes (default: 4096)
- `ic10.useRemoteLanguageServer`: Use remote LSP server for development

## Issues & Feedback

Found a bug or have a suggestion? Please [open an issue on GitHub](https://github.com/Anexgohan/Stationeers-ic10/issues).

## License

This extension is released under the [MIT License](https://opensource.org/licenses/MIT).

## Credits

This project builds upon the excellent work of:
- **[Xandaros](https://github.com/Xandaros)** for the original `ic10lsp` language server
- **[awilliamson](https://github.com/awilliamson)** for the original `ic10-language-support` VSCode extension