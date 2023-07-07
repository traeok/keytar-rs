# Keytar VSCode extension

This test VS Code extension lets you test 4 credential providers:
* VSCode Keytar - the Keytar shim in VSCode that will be removed in Aug 2023
* Bundled Keytar - the Node.js Keytar package bundled into the extension
* Bundled Keytar-RS - the rewrite of Keytar in Rust bundled into the extension
* Secret Storage - the new secret storage API that VSCode is recommending

## Testing

1. Clone the `keytar-rs` repo
2. Run `yarn` to install top-level dependencies and build the Keytar native binary
3. Change directory to `vscode`
4. Run `yarn install` to install dependencies for the VSCode extension
5. Run the extension by either:
  - In VS Code, run the extension with the debugger
  - Run `yarn run vscode:build` and install the extension
6. Test the commands defined by the extension:
  * Keytar: Get Password
  * Keytar: Set Password
  * Keytar: Delete Password

**Note:** The extension tries to create a secret called "Zowe/test_secret", so it should be visible under that name in Windows Credential Manager or macOS Keychain.
