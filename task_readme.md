# Below are instruction on what to do copy paste to seed the chat:  
@task_readme.md
@anex_stationeers_Instructions_Functions.md
read the Instructions and perform the task numbers as requested and described in the "task_readme.md"
Do Task Number 1 after that ask what to do  

# Start of the instructions:
## Use this information and do the following:    
1. git clone of the following repos included under git-clone directory:  
  - ic10lsp - https://github.com/Xandaros/ic10lsp
  This is a very comprehensive language server implementation. Here's a breakdown of what I've learned from the code and how it relates to our goal of improving the linter:
   * Tree-sitter: The server uses tree-sitter for parsing the IC10 code. This is a powerful parser generator that creates a concrete syntax tree. This is great news because it means we can write queries to find
     specific patterns in the code, which is the foundation of a good linter.
   * Diagnostics (Linting): The run_diagnostics function is the heart of the linter. It's responsible for publishing diagnostics (errors and warnings) to the client. It currently checks for:
       * Syntax errors
       * Invalid instructions
       * Type mismatches
       * Code exceeding line and column limits
       * Absolute jumps to line numbers
       * Use of literal numbers for batch and reagent modes
   * Configuration: The server supports configuration options like max_lines, max_columns, and various warnings. This is good because it allows users to customize the linter's behavior.
   * Code Actions: The code_action function provides quick fixes for some of the diagnostics. This is a great feature that we can expand upon.

  - ic10-language-support - https://github.com/awilliamson/ic10-language-support
  full source code for the ic10-language-support extension. Here's a quick overview of the important files:
   * package.json: This is the manifest file for the extension, which I've already examined.
   * src/extension.ts: This is the main entry point for the extension's code. It's responsible for activating and deactivating the extension, and for starting the language server.
   * syntaxes/ic10.tmLanguage.json: This file defines the syntax highlighting rules for the IC10 language
   examine the extension.ts file
    Here's the summary:
   * Language Server Executable: The extension looks for ic10lsp.exe on Windows and ic10lsp on other platforms in a bin directory inside the extension's installation directory.
   * Local vs. Remote: The extension can connect to a remote language server if the ic10.useRemoteLanguageServer setting is true. Otherwise, it runs the ic10lsp executable locally.
   * Language Client: The extension uses the vscode-languageclient library to communicate with the language server. It sends notifications to the server when the configuration changes and provides commands to
     restart the server and check its version.

2. extracted @ashleywilliamson.ic10-language-support-0.7.4 also which is the actual extention that vscode downloads, ive copied it in our working directory over from "C:\Users\Anex\AppData\Roaming\Code\CachedExtensionVSIXs", this package was extracted as a zip to "extracted-vsix" directory
  - VSIX contains the same files as the ic10-language-support repository I cloned earlier. The key takeaway is that the bin directory contains the pre-compiled language server
  executables (ic10lsp.exe and ic10lsp).

3. this is documentation of what ic10 code is and works @anex_stationeers_Instructions_Functions.md

## Introduction:
ic10 is a simple language that is used to control and interact the devices in the game stationeers.

## Your task:  
Additional Information or request for task:  

Task Number:
1. Evaluate each repo to learn how it functions and how it can be used to creat our linter.
2. Create a linter that can be used to lint IC10 code for VS Code.
3. we need to add support for some new code that was added to the game recently. that includes support for 
    - addressing devices with sbn and lbn  

4. Create a vscode extension that can use the linter to lint IC10 code.  



