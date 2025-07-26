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

-   **File:** `src/additional_features.rs`
    -   **Purpose:** Contains advanced analysis features like register usage tracking and variable analysis.
    -   **Edit this file to:**
        -   Add new register analysis features.
        -   Implement additional static analysis checks.
        -   Extend variable usage tracking capabilities.

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

2.  **For Development/Testing:** Build the project in debug mode for faster compilation:
    ```bash
    cargo build
    ```
    The debug binary will be located at `anex-ic10/ic10lsp/target/debug/ic10lsp.exe`.

3.  **For Production/Release:** Build the project in release mode for optimized performance:
    ```bash
    cargo build --release
    ```
    The release binary will be located at `anex-ic10/ic10lsp/target/release/ic10lsp.exe`.

4.  **Quick Testing:** You can run the language server directly for testing:
    ```bash
    cargo run
    ```

### Step 2: Update the Extension Binary

The VSCode extension needs the latest compiled linter binary to function correctly.

1.  After the build completes, copy the new binary into the extension's `bin` directory. The following command should be run from the `anex-ic10/ic10lsp` directory you are currently in:
    
    **For Development/Testing (debug build):**
    ```bash
    copy target\debug\ic10lsp.exe ..\anex-ic10-language-support\bin\ic10lsp.exe /Y
    ```
    
    **For Production/Release (release build):**
    ```bash
    copy target\release\ic10lsp.exe ..\anex-ic10-language-support\bin\ic10lsp.exe /Y
    ```
    
    *(Note: For other operating systems, you would copy the `ic10lsp` file instead of `ic10lsp.exe`)*

### Step 3: Build the VSCode Extension

After making changes to the extension's TypeScript code (in `anex-ic10/anex-ic10-language-support/src`), you need to compile it.

1.  Navigate to the extension's directory:
    ```bash
    cd anex-ic10/anex-ic10-language-support
    ```

2.  If you haven't already, or if you've changed dependencies in `package.json`, install the npm packages:
    ```bash
    npm install
    ```

3.  **For Development:** Build with source maps for debugging:
    ```bash
    npm run esbuild
    ```

4.  **For Development with Watch Mode:** Automatically rebuild on file changes:
    ```bash
    npm run esbuild-watch
    ```

5.  **For Production:** Build optimized version for packaging:
    ```bash
    npm run vscode:prepublish
    ```

### Step 4: Package the VSCode Extension

To create a distributable `.vsix` file:

1.  Ensure you're in the extension directory and have run a production build:
    ```bash
    cd anex-ic10/anex-ic10-language-support
    npm run vscode:prepublish
    ```

2.  Package the extension into a `.vsix` file:
    ```bash
    npx vsce package
    ```

This will create a file named `anex-ic10-language-support-x.x.x.vsix` in the `anex-ic10/anex-ic10-language-support` directory. This file can be installed in VSCode through the "Extensions" view by selecting "Install from VSIX...".

## Testing Your Changes

### Testing the Language Server

Before packaging the extension, you can test the language server directly:

1. **Unit Tests:** Run the Rust tests to ensure core functionality works:
   ```bash
   cd anex-ic10/ic10lsp
   cargo test
   ```

2. **Manual Testing:** Use the test files in `anex-ic10/dev/testing/` to verify new features:
   ```bash
   # Test register usage analysis with the provided test file
   # Open register_usage_test.ic10 in VSCode with your updated extension
   ```

### Testing Register Usage Analysis Feature

The register usage analysis feature detects:
- **Unused registers:** Registers that are never assigned or read
- **Assigned but never read:** Registers that get values but are never used
- **Read before assign:** Registers that are used before being given a value

**Supported Assignment Operations:**
The analysis correctly recognizes these IC10 instructions as assignments to the first register operand:
- Basic operations: `move`, `add`, `sub`, `mul`, `div`, `mod`, `max`, `min`
- Math functions: `abs`, `ceil`, `floor`, `round`, `sqrt`, `trunc`, `exp`, `log`
- Trigonometry: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- Bitwise: `and`, `or`, `xor`, `nor`, `not`, `sla`, `sll`, `sra`, `srl`
- Load operations: `l`, `lb`, `lr`, `ls`, `lbn`, `lbs`, `lbns`, `lhz`, `lhs`
- Stack operations: `peek`, `pop`
- Comparison: `seq`, `seqz`, `sge`, `sgez`, `sgt`, `sgtz`, `sle`, `slez`, `slt`, `sltz`, `sna`, `snaz`, `sne`, `snez`
- Other: `select`, `sap`, `sapz`, `sdns`, `sdse`, `rget`, `alias`

**Special Cases Handled:**
- **`jal` instructions:** Automatically assigns return address to `ra` register
- **Alias tracking:** Properly links register usage through aliases (e.g., `alias temp r0`)
- **Batch operations:** `lbn`, `lbs`, `lbns` correctly detected as assignments

**Test Files:**
1. **Basic usage:** `anex-ic10/dev/testing/register_usage_test.ic10`
   - `pressure` (r1) - assigned but never read
   - `unused_reg` (r2) - never used
   - `r4` - read before being assigned

2. **Alias handling:** `anex-ic10/dev/testing/simple_alias_test.ic10`
   - Tests alias assignment and read detection

3. **Complex real-world code:** `anex-ic10/dev/testing/example_furnace_01.ic10`
   - Advanced furnace control with batch operations
   - Tests `lbn` instruction handling and `jal`/`ra` usage

4. **Airlock control:** `anex-ic10/dev/testing/Anex-2025_Airlock_enhanced_mk3.ic10`
   - Complex state machine with extensive alias usage

### Development Workflow

For active development, use this streamlined workflow:

1. **Start with watch mode for the extension:**
   ```bash
   cd anex-ic10/anex-ic10-language-support
   npm run esbuild-watch
   ```

2. **Make changes to the Rust code and rebuild:**
   ```bash
   cd anex-ic10/ic10lsp
   cargo build
   copy target\debug\ic10lsp.exe ..\anex-ic10-language-support\bin\ic10lsp.exe /Y
   ```

3. **Reload VSCode window:** Press `Ctrl+Shift+P` → "Developer: Reload Window" to test changes

## Troubleshooting

### Common Issues

1. **"Language server not found" error:**
   - Ensure the `ic10lsp.exe` binary exists in `anex-ic10/anex-ic10-language-support/bin/`
   - Check that the binary was copied from the correct target directory (debug vs release)

2. **Extension not loading:**
   - Run `npm run esbuild` to ensure TypeScript compilation succeeded
   - Check the VSCode Developer Console for error messages

3. **Register analysis not working:**
   - Verify the `additional_features.rs` module is properly compiled
   - Check that diagnostics are enabled in VSCode settings
   - Ensure register usage analysis is running (should see warnings in Problems panel)

4. **False "read before assign" errors:**
   - Check if the instruction is in the supported assignment operations list
   - Common issue: `lbn`, `lbs`, `lbns` not recognized as assignments (should be fixed in v1.2.0+)
   - For `ra` register: ensure `jal` instructions are being detected properly

5. **False "unused register" warnings for aliases:**
   - Verify aliases are defined before use: `alias temp r0`
   - Check that both direct register usage (`r0`) and alias usage (`temp`) are tracked
   - Ensure alias mapping is working in the `TypeData` structure

6. **Binary compatibility issues:**
   - Ensure you're using the correct binary for your platform (Windows: `.exe`, Linux/Mac: no extension)
   - Rebuild the Rust project if you've switched between debug/release modes

## Version History & Recent Fixes

### Version 1.2.0+ - Register Usage Analysis

**Major Features Added:**
- **Register Usage Analysis:** Detects unused registers, read-before-assign errors, and assigned-but-never-read warnings
- **Alias Support:** Properly tracks register usage through aliases
- **Comprehensive Instruction Support:** Recognizes 40+ IC10 instructions as assignment operations

**Key Fixes Applied:**

1. **Batch Instruction Support (Critical Fix):**
   - Added support for `lbn`, `lbs`, `lbns`, `lhz`, `lhs` instructions
   - **Issue:** `lbn r0 device property` was not recognized as assignment to r0
   - **Fix:** Extended `is_assignment_operation()` to include batch load instructions
   - **Impact:** Eliminates false "read before assign" errors in complex IC10 code

2. **JAL/RA Register Handling:**
   - `jal` instructions now properly mark `ra` as assigned
   - **Issue:** `j ra` was flagged as "read before assign" even after `jal` call
   - **Fix:** Added `detect_jal_ra_assignments()` method
   - **Impact:** Proper support for function calls and returns

3. **Alias Resolution Improvements:**
   - Enhanced alias-to-register mapping logic
   - **Issue:** Aliases like `alias temp r0` weren't linking usage properly
   - **Fix:** Improved operand parsing and identifier resolution
   - **Impact:** Accurate tracking of aliased register usage

4. **Line Ordering Logic:**
   - Improved assignment vs read timing detection
   - **Issue:** False positives due to incorrect line number comparison
   - **Fix:** Proper sorting of assignments and reads by line number
   - **Impact:** More accurate read-before-assign detection

**Testing Coverage:**
- ✅ Basic register operations (move, arithmetic)
- ✅ Batch network operations (lbn, lbs, lbns)
- ✅ Function calls (jal/ra)
- ✅ Complex alias usage
- ✅ Real-world IC10 scripts (furnace control, airlock systems)

**Known Limitations:**
- Static analysis only - doesn't track runtime register values
- No flow control analysis (branches, loops)
- Self-referencing operations (e.g., `mul r0 r0 2`) are considered valid if r0 was previously assigned

```