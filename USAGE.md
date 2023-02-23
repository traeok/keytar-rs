# User Documentation

## What is `keytar-rs`?

`keytar-rs` is a cross-platform library meant to interact with OS (operating system) credential storage. `keytar-rs` is written in Rust, and uses other Rust libraries to interface with credential storage APIs (application programming interfaces). It was designed to be a drop-in replacement for `node-keytar`, a Microsoft (formerly GitHub under "Atom") project that was archived on December 15th, 2022. For context, `node-keytar` has widespread use in multiple projects with over 500k weekly downloads - this library was created to avoid long-term conflicts/vulnerabilities that may arise with `node-keytar` now that it is no longer maintained.

## Why switch to `keytar-rs`?

By continuing to use `node-keytar`, it opens up the user to future problems with the library itself or its dependencies. Until Microsoft provides an update to the status of `keytar` on NPM, it is unknown whether the package will continue to be supported. As a result, there was a demand for a replacement that can function identically to the original module.

As `keytar-rs` was modeled after `node-keytar`, the same operations can be performed in credential storage:

- Storing credentials
- Retrieving credentials
- Searching for passwords based on a matching label
- Searching for matching credentials based on a prefix/query
- Deleting credentials

**Currently, there are no breaking changes** between the use of `node-keytar` and `keytar-rs`. This is intended by design, and the library will be maintained with that principle in mind. As a result, `keytar-rs` is a substitute for the original `node-keytar` module.

For the end-user, there will be no noticeable change to their operations. Existing credentials stored and retrieved with `node-keytar` will continue to work with `keytar-rs`. Existing functionality dependent upon `node-keytar` will continue to work when `keytar-rs` is integrated.

## Usage (developers)

From a developer's perspective, one can simply update existing Node.js projects to import from `keytar-rs` instead of `node-keytar`, allowing for a straightforward transition. All functions previously exported in `node-keytar` will be available in `keytar-rs`. Simply add `keytar-rs` to your project using `npm` or `yarn`:

- `npm install git+https://github.com/traeok/keytar-rs.git`
- `yarn add git+https://github.com/traeok/keytar-rs.git`

Importing a function from `keytar-rs` is identical to the `node-keytar` import process:

```ts
import { 
    deletePassword, 
    findCredentials, 
    findPassword, 
    getPassword, 
    setPassword 
} from "@traeok/keytar-rs";
// Or import all functions under a namespace...
import * as keytar from "@traeok/keytar-rs";
```

After the desired functions are imported, feel free to use them in the same fashion as the `node-keytar` functions. For the examples below, `async/await` keywords are used, but the functions can also be used with `.then/.catch` promise blocks:

```ts
getPassword("TestService", "AccountA")
.then((pw) => {
    console.log("The password for TestService/AccountA is:", pw);
})
.catch((err) => {
    console.error("An error occurred!", err.message);
});
```

**Examples:**

```ts
// Set a password with a given service and account name
// Password will be stored under <service>/<account>
await setPassword("TestService", "AccountA", "Apassword");

// Get a password, given a service and account name
await getPassword("TestService", "AccountA");

// Find credentials based on a matching label
await findCredentials("TestService");

// Find password that matches a service and account
await findPassword("TestService/AccountA");

// Delete a credential w/ the provided service and account name
await deletePassword("TestService", "AccountA");
```

**Demo:**

![keytar-rs demo](./DEMO.svg)