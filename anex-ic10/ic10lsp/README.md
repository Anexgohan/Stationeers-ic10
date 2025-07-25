# IC10LSP

A comprehensive language server for the IC10 MIPS-like assembly language used in the game Stationeers.

## Features

### Core Language Support
- **Syntax Highlighting**: Full semantic token support for instructions, registers, devices, and labels
- **Code Completion**: Intelligent completions for:
  - IC10 instructions with parameter hints
  - Device hash names with fuzzy search
  - Register and device references
- **Hover Information**: Detailed tooltips showing:
  - Instruction documentation and signatures
  - Device hash values with display names
  - Register and device information
- **Go-to-Definition**: Navigate to labels and definitions
- **Diagnostics**: Real-time error detection for:
  - Syntax errors and invalid instructions
  - Code length limits (lines, columns, bytes)
  - Type checking for parameters

### Device Hash Support
- **84+ Device Types**: Comprehensive support for Stationeers devices
- **Fuzzy Search**: Type partial names like "Battery" to find all battery devices
- **Inlay Hints**: Shows device display names with arrow notation (→)
- **Hash Tooltips**: Display hash values in both decimal and hexadecimal formats

### Advanced Features
- **Code Actions**: Quick fixes for common issues
- **Signature Help**: Parameter guidance while typing instructions
- **Length Validation**: Configurable limits for IC10 hardware constraints
- **Unicode Support**: Built-in arrow symbols and special characters

![Demo](demo.gif)

## Configuration

The language server exposes the following configuration options:

| Key                         | Description                                      | Default |
| --------------------------- | ------------------------------------------------ | ------- |
| max_lines                   | Maximum number of lines                          | 128     |
| max_columns                 | Maximum number of columns                        | 90      |
| max_bytes                   | Maximum bytes (IC10 hardware limit)             | 4096    |
| warnings.overline_comment   | Emit warnings for comments past line limit      | true    |
| warnings.overcolumn_comment | Emit warnings for comments past column limit    | true    |

## Commands

The language server exposes the following commands:

| Command | Description                                            |
| ------- | ------------------------------------------------------ |
| version | Show a message with the version of the language server |
| restart | Restart the language server                            |

## Supported Devices

The language server includes hash mappings for 84+ Stationeers devices across 9 categories:
- Access Control (Airlocks, Blast Doors)
- Atmospheric (Pumps, Valves, Regulators, Coolers)
- Displays (LED Displays, Consoles, Graph Displays)
- Hydroponics (Growing equipment, Harvesters)
- Lighting (Wall Lights, Flashing Lights)
- Logic (Processors, Memory, Sorters, Transmitters)
- Manufacturing (Furnaces, Centrifuges, Autolathes)
- Power (Batteries, Generators, Solar Panels)
- Sensors (Gas Sensors, Motion Sensors, Cameras)
- Storage (Tanks, Chutes, Stackers)

See [Device Hash Reference](../../documentation/hashes_ids.md) for complete list.
