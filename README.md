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
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓</td>
    </tr>
    <tr>
        <td><i>zLinux</i></td>
        <td><i>s390x</i></td>
        <td colspan="4"><i>Not yet implemented</i></td>
    </tr>
</table>

## Features

keytar-rs supports the following operations within credential storage:

- [x] **Set** a credential
- [x] **Retrieve** a credential 
- [x] **Find all credentials** with matching attributes 
- [x] **Find a password** with matching attributes

## Usage

### deletePassword

```ts
function deletePassword(service: string, account: string, password: string) -> Promise<bool>
```

### findCredentials

```ts
interface Credential {
  username: string;
  password: string;
};

function findCredentials(service: string) -> Promise<Array<Credential>>
```

### findPassword

```ts
function findPassword(service: string, account: string) -> Promise<string>
```

### getPassword

```ts
function getPassword(service: string, account: string) -> Promise<string>
```

### setPassword

```ts
function setPassword(service: string, account: string, password: string) -> Promise<bool>
```

*Docs coming soon!*

