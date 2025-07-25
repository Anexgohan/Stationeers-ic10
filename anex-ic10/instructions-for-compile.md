# How to Compile and Package the IC10 Linter Extension

This guide provides the steps to modify, compile, and package the IC10 linter and its corresponding VSCode extension.

## Prerequisites

Before you begin, ensure you have the following installed:

1.  **Node.js and npm:** Required for managing the VSCode extension's dependencies and packaging. You can download it from [https://nodejs.org/](https://nodejs.org/).
2.  **Rust and Cargo:** Required for compiling the `ic10lsp` language server. You can install it using `rustup` from [https://rustup.rs/](https://rustup.rs/).
3.  **VSCE:** The command-line tool for packaging VSCode extensions. You can install it globally via npm:
    ```bash
    npm install --global @vscode/vsce
    ```

## Where to Make Changes

To add or modify features, you'll need to edit specific files within the project structure. Here’s a guide to the most common files and what they control:

### Core Linter Logic (in `anex-ic10/ic10lsp/`)

-   **File:** `src/instructions.rs`
    -   **Purpose:** Defines all known IC10 instructions, their parameters (signatures), and documentation for hover-over tooltips.
    -   **Edit this file to:**
        -   Add a new instruction to the `INSTRUCTIONS` map.
        -   Change the parameters of an existing instruction.
        -   Add new `LOGIC_TYPES`, `SLOT_LOGIC_TYPES`, or other language constants.

-   **File:** `src/main.rs`
    -   **Purpose:** Contains the main language server logic, including all diagnostic checks.
    -   **Edit this file to:**
        -   Add new linting rules or validation checks inside the `run_diagnostics` function.
        -   Implement new features like code actions (quick fixes) in the `code_action` function.
        -   Handle new configuration options sent from the VSCode client.

-   **File:** `src/device_hashes.rs`
    -   **Purpose:** Device name to hash mappings for HASH() function tooltips and inline hints.
    -   **Edit this file to:**
        -   Add new device mappings to the `DEVICE_NAME_TO_HASH` map.
        -   Update display names in the `HASH_TO_DISPLAY_NAME` reverse mapping.

-   **File:** `src/hash_utils.rs`
    -   **Purpose:** Utility functions for HASH() argument extraction and CRC32 computation.
    -   **Edit this file to:**
        -   Modify HASH() parsing logic.
        -   Update device name lookup functions.

### VSCode Extension (in `anex-ic10/anex-ic10-language-support/`)

-   **File:** `package.json`
    -   **Purpose:** The manifest file for the extension. It controls how the extension appears and behaves within VSCode.
    -   **Edit this file to:**
        -   Change the extension's name (`displayName`), version, author (`publisher`), or description.
        -   Add new user-configurable settings under `contributes.configuration.properties`.
        -   Add new commands to the VSCode Command Palette.

-   **File:** `syntaxes/ic10.tmLanguage.json`
    -   **Purpose:** Defines the syntax highlighting rules for the IC10 language.
    -   **Edit this file to:**
        -   Add new keywords, constants, or language constructs to ensure they are colored correctly in the editor.

-   **File:** `src/extension.ts`
    -   **Purpose:** The main entry point for the extension's client-side code.
    -   **Edit this file to:**
        -   Change how the language server is started or stopped.
        -   Implement the client-side logic for new commands you've added in `package.json`.

## Compilation and Packaging Workflow

The project is split into two main parts:

*   `ic10lsp`: The language server written in Rust. This is where the core linting logic resides.
*   `anex-ic10-language-support`: The VSCode extension written in TypeScript, which acts as a client to the language server.

Follow these steps to build and package the extension after making changes.

### Step 1: Modify the Linter (Rust)

If you've made changes to the linter's logic in the `anex-ic10/ic10lsp/src` directory, you must recompile the binary.

1.  Open a terminal and navigate to the linter's directory:
    ```bash
    cd anex-ic10/ic10lsp
    ```

2.  Build the project in release mode. This will create an optimized executable.
    ```bash
    cargo build --release
    ```

3.  The new binary will be located at `anex-ic10/ic10lsp/target/release/ic10lsp.exe`.

### Step 2: Update the Extension Binary

The VSCode extension needs the latest compiled linter binary to function correctly.

1.  After the build completes, copy the new binary into the extension's `bin` directory. The following command should be run from the `anex-ic10/ic10lsp` directory you are currently in:
    ```bash
    copy target\release\ic10lsp.exe ..\anex-ic10-language-support\bin\ic10lsp.exe /Y
    ```
    *(Note: For other operating systems, you would copy the `ic10lsp` file instead of `ic10lsp.exe`)*


### Step 3: Package the VSCode Extension

After updating the binary or making changes to the extension's TypeScript code (in `anex-ic10/anex-ic10-language-support/src`), you need to package the extension.

1.  Navigate to the extension's directory:
    ```bash
    cd anex-ic10/anex-ic10-language-support
    ```

2.  If you haven't already, or if you've changed dependencies in `package.json`, install the npm packages:
    ```bash
    npm install
    ```

3.  Package the extension into a `.vsix` file:
    ```bash
    npx vsce package
    ```

This will create a file named `anex-ic10-language-support-x.x.x.vsix` in the `anex-ic10/anex-ic10-language-support` directory. This file can be installed in VSCode through the "Extensions" view by selecting "Install from VSIX...".

```