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
function getNodeModule<T>(moduleName: string): T | undefined {
  const r = typeof __webpack_require__ === 'function' ? __non_webpack_require__ : require;
  try {
    return r(`${vscode.env.appRoot}/node_modules.asar/${moduleName}`);
  } catch (err) {
    // Not in ASAR.
  }
  try {
    return r(`${vscode.env.appRoot}/node_modules/${moduleName}`);
  } catch (err) {
    // Not available.
  }
  return undefined;
}

function initVscodeKeytar(): ICredentialProvider {
	return getNodeModule("keytar") as ICredentialProvider;
}

function initBundledKeytar(): ICredentialProvider {
	return require("keytar");
}

function initBundledKeytarRs(): ICredentialProvider {
	return require("@traeok/keytar-rs");
}

function initSecretStorage(context: vscode.ExtensionContext): ICredentialProvider {
	return {
		getPassword: async (service: string, account: string) => {
			return (await context.secrets.get(`${service}/${account}`)) ?? null;
		},
		setPassword: async (service: string, account: string, password: string) => {
			return await context.secrets.store(`${service}/${account}`, password);
		},
		deletePassword: async (service: string, account: string) => {
			await context.secrets.delete(`${service}/${account}`);
			return true;
		}
	};
}

const CREDENTIAL_PROVIDER_MAP = {
	"VSCode Keytar": initVscodeKeytar,
	"Bundled Keytar": initBundledKeytar,
	"Bundled Keytar-RS": initBundledKeytarRs,
	"Secret Storage": initSecretStorage
};
const TEST_SERVICE = "Zowe";
const TEST_ACCOUNT = "test_secret";
let OUR_CONTEXT: vscode.ExtensionContext;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	OUR_CONTEXT = context;
	console.log('Congratulations, your extension "helloworld" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	const disposables = [];
	disposables.push(vscode.commands.registerCommand("helloworld.getPassword", async () => {
		const password = await getPassword();
		return password;
	}));
	disposables.push(vscode.commands.registerCommand("helloworld.setPassword", async () => {
		const set = await setPassword();
		return set;
	}));
	disposables.push(vscode.commands.registerCommand("helloworld.deletePassword", async () => {
		const deleted = await deletePassword();
		return deleted;
	}));

	context.subscriptions.push(...disposables);
}

// This method is called when your extension is deactivated
export function deactivate() {}

async function deletePassword(): Promise<boolean> {
	try {
		await initVscodeKeytar().deletePassword(TEST_SERVICE, TEST_ACCOUNT);
		await initSecretStorage(OUR_CONTEXT).deletePassword(TEST_SERVICE, TEST_ACCOUNT);
		return true;
	} catch (err) {
		vscode.window.showErrorMessage(`Failed to delete password: ${err}`);
		return false;
	}
}

async function setPassword() : Promise<boolean> {
	let error = false;
	await vscode.window.showQuickPick(Object.keys(CREDENTIAL_PROVIDER_MAP),
		{ canPickMany: false, title: "Select a credential provider" }).then(async (result) => {
			if (result == null) {
				error = true;
				return;
			}
			let credentialProvider: ICredentialProvider;
			try {
				credentialProvider = (CREDENTIAL_PROVIDER_MAP as any)[result](OUR_CONTEXT);
			} catch (err) {
				vscode.window.showErrorMessage(`Failed to initialize ${result}: ${err}`);
				error = true;
				return;
			}
			await vscode.window.showInputBox({ title: "Enter a secret value", value: "password" }).then(async (password) => {
				try {
					await credentialProvider.setPassword(TEST_SERVICE, TEST_ACCOUNT, password as string);
				} catch (err) {
					vscode.window.showErrorMessage(`Failed to set password: ${err}`);
					error = true;
				}
		});
	});
	return error;
};

async function getPassword(): Promise<string> {
	let thepassword: string = "";
	await vscode.window.showQuickPick(Object.keys(CREDENTIAL_PROVIDER_MAP),
		{ canPickMany: false, title: "Select a credential provider" }).then(async (result) => {
			if (result == null) {
				return;
			}
			let credentialProvider: ICredentialProvider;
			try {
				credentialProvider = (CREDENTIAL_PROVIDER_MAP as any)[result](OUR_CONTEXT);
			} catch (err) {
				vscode.window.showErrorMessage(`Failed to initialize ${result}: ${err}`);
				return;
			}
			try {
				const password = await credentialProvider.getPassword(TEST_SERVICE, TEST_ACCOUNT);
				vscode.window.showInformationMessage(`${result}: ${password}`);
				thepassword = password ? password : "";
			} catch (err) {
				vscode.window.showErrorMessage(`Failed to get password: ${err}`);
				return;
			}
	});
	return thepassword;
};