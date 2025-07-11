# IC10 Language Support for Stationeers

## Downloads

Download the latest VS Code IC10 Extension:
- **[Windows](https://github.com/Anexgohan/Stationeers-ic10/releases/latest/download/ic10-language-support-Windows.vsix)**
- **[Linux](https://github.com/Anexgohan/Stationeers-ic10/releases/latest/download/ic10-language-support-Linux.vsix)**
- **[macOS](https://github.com/Anexgohan/Stationeers-ic10/releases/latest/download/ic10-language-support-macOS.vsix)**

---

This Visual Studio Code extension provides syntax highlighting, IntelliSense, and other language support features for the IC10 MIPS-like language used in the game Stationeers. Uses the Language Server [ic10lsp](https://github.com/Anexgohan/Stationeers-ic10/tree/main/ic10lsp)

## Features

- Syntax highlighting for IC10 language files (`.ic10`)
- Code autocompletion
- Hover information
- Signature help
- Go to definition
- Diagnostics

## Installation

1. Open Visual Studio Code.
2. Press `Ctrl+P` to open the Quick Open dialog.
3. Type `ext install vsix`
4. Select the latest version of the extension and press `Enter`.
5. Once the extension is installed, you can start using it by opening `.ic10` files.

![how-to-install-in-vscode](how-to-install-in-vscode.png)

## Usage

After installing the extension, you can use it by opening any `.ic10` file in Visual Studio Code. The extension will automatically activate and provide syntax highlighting and language features for your IC10 MIPS-like code.

![Example showing autocompletions for ic10 and hover text](./images/example_working.png)

## Contributing

If you find any issues or have suggestions for improvements, please [open an issue on GitHub](https://github.com/Anexgohan/Stationeers-ic10/issues).

## License

This extension is released under the [MIT License](https://opensource.org/licenses/MIT). See the [LICENSE](https://github.com/Anexgohan/Stationeers-ic10/blob/master/LICENSE) file for more information.

## Special Thanks

This project is a fork and modification of the great work done by the following individuals:

- **[Xandaros](https://github.com/Xandaros)** for the original `ic10lsp` language server: [https://github.com/Xandaros/ic10lsp](https://github.com/Xandaros/ic10lsp)
- **[awilliamson](https://github.com/awilliamson)** for the original `ic10-language-support` VSCode extension: [https://github.com/awilliamson/ic10-language-support](https://github.com/awilliamson/ic10-language-support)