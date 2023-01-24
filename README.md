# keytar-rs

keytar-rs is a native Node.js module for accessing and managing OS credential storage. It is a Rust approach to the npm library [node-keytar](https://github.com/atom/node-keytar).

## Compatibility

<table>
    <tr>
        <th colspan="2">OS / Architecture</th>
        <th colspan="4">Node.js Version</th>
    </tr>
    <tr>
        <td colspan="2"></td>
        <td>v12</td>
        <td>v14</td>
        <td>v16</td>
        <td>v18</td>
    </tr>
    <tr>
        <td><b>Windows</b></td>
        <td>x64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td></td>
        <td>x86</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td></td>
        <td>arm64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td><b>macOS</b></td>
        <td>x64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td></td>
        <td>aarch64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td><b>Linux (gnu)</b></td>
        <td>x64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td></td>
        <td>aarch64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td><b>Linux (musl)</b></td>
        <td>x64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td></td>
        <td>aarch64</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td><b>FreeBSD</b></td>
        <td>x64</td>
        <td colspan="4">⌛ <i>resolving some issues</i></td>
    </tr>
    <tr>
        <td><i>zLinux</i></td>
        <td><i>s390x</i></td>
        <td colspan="4"><i>💭 Not yet implemented</i></td>
    </tr>
</table>

## Features

keytar-rs supports the following operations within credential storage:

- [x] **Set** a credential
- [x] **Retrieve** a credential 
- [x] **Find all credentials** with matching attributes 
- [x] **Find a password** with matching attributes

Some benefits to using keytar-rs:

- [x] **Cross-platform support** makes for straight-forward secrets management
- [x] **Existing OS credentials are supported** out-of-the-box
- [x] **Avoids memory allocation** - memory only allocated as needed for OS-specific APIs

## Node API documentation

### deletePassword

Deletes a password with matching `service` and `account` parameters.

**Returns:** Whether the password was deleted successfully.

```ts
function deletePassword(service: string, account: string) -> Promise<boolean>
```

### findCredentials

Finds all credentials with a matching `service` parameter.

**Returns:** An array of `Credential` objects, containing the `account` and `password` for each credential that is found within `service`.

```ts
interface Credential {
  account: string;
  password: string;
};

function findCredentials(service: string) -> Promise<Array<Credential>>
```

### findPassword

Finds a password with a matching `service` and `account` parameter.

**Returns:** The first password found in `<service>/<account>`, or `''` (empty string) if not found.

```ts
function findPassword(service: string, account: string) -> Promise<string>
```

### getPassword

Gets a password with a matching `service` and `account` parameter.

**Returns:** The password stored under `<service>/<account>`.

```ts
function getPassword(service: string, account: string) -> Promise<string>
```

### setPassword

Stores a password with the given `service`, `account`, and `password`.

**Returns:** Whether the password was stored successfully.

```ts
function setPassword(service: string, account: string, password: string) -> Promise<boolean>
```