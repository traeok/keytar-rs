#!/usr/bin/env node

const helpText = `Keytar CLI for getting/setting/deleting credentials.

Usage:
  keytar get <service> [account]
  keytar set <service> <account> [-p <string>]
  keytar delete <service> <account> [-y]
  keytar -h | --help
  keytar -v | --version

Options:
  -h --help       Show this screen.
  -p --password   Provide password value.
  -v --version    Show version.
  -y --yes        Skip confirmation prompt.
`;

(async () => {
    const argv = require("minimist")(process.argv.slice(2));

    if (process.argv.length === 2 || argv.h || argv.help) {
        console.log(helpText);
        process.exit();
    } else if (argv.v || argv.version) {
        console.log(require("./package.json").version);
        process.exit();
    }

    const keytar = require("..");
    const command = argv._[0]?.toLowerCase();
    const service = argv._[1];
    const account = argv._[2];
    let password = argv.p || argv.password;

    switch (command) {
        case "get":
            if (account) {
                password = await keytar.getPassword(service, account);
            } else {
                const credentials = await keytar.findCredentials(service);
                password = {};
                credentials.forEach((credential) => {
                    password[credential.account] = credential.password;
                });
            }
            console.log(password);
            break;
        case "set":
            if (password == null) {
                password = await require("password-prompt")(`Enter password: `, { method: "hide" });
            }
            await keytar.setPassword(service, account, password);
            break;
        case "delete":
            let ok = argv.y || argv.yes;
            if (!ok) {
                ok = await require("yesno")({
                    question: "Are you sure you want to delete this password [y/N]?",
                    defaultValue: false
                });
            }
            if (ok) {
                const deleted = await keytar.deletePassword(service, account);
                process.exitCode = deleted ? 0 : 1;
            }
            break;
        default:
            throw new Error(`Invalid command: "${command}"`);
    }
})().catch((error) => {
    console.error(error);
    process.exit(1);
});
