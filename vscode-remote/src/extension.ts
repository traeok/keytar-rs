// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

interface ICredentialProvider {
	getPassword: (service: string, account: string) => Promise<string | null>;
	setPassword: (service: string, account: string, password: string) => Promise<void>;
	deletePassword: (service: string, account: string) => Promise<boolean>;
}

declare const __webpack_require__: typeof require;
declare const __non_webpack_require__: typeof require;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "helloworld2" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	const disposables = [];
	disposables.push(vscode.commands.registerCommand("helloworld2.getPassword", () => {
		vscode.commands.executeCommand("helloworld.getPassword").then( async (password) => {
			vscode.window.showInformationMessage(`Password: ${password}`);
		});
	}));
	disposables.push(vscode.commands.registerCommand("helloworld2.setPassword", () => {
		vscode.commands.executeCommand("helloworld.setPassword").then( async (set) => {
			vscode.window.showInformationMessage(`The password was ${set ? "not " : ""}set`);
		});
	}));
	disposables.push(vscode.commands.registerCommand("helloworld2.deletePassword", async () => {
		vscode.commands.executeCommand("helloworld.deletePassword").then (async (deleted) => {
			vscode.window.showInformationMessage(`The password was ${deleted ? "not " : ""}deleted`);
		});
	}));

	context.subscriptions.push(...disposables);
}

// This method is called when your extension is deactivated
export function deactivate() {}
