import test from "ava";
import {
  deletePassword,
  findCredentials,
  findPassword,
  getPassword,
  setPassword,
} from "../index.js";

// generate a number in range [min, max)
const randomInt = (min, max) => {
  return Math.floor(Math.random() * (max - min)) + min;
};

// generate random ASCII string of length "len"
// (could use constants instead, but this should better emulate real-world scenarios)
const randomAsciiString = (len) => {
  let str = "";
  for (let i = 0; i < len; i++) {
    str = str.concat(String.fromCharCode(randomInt(97, 123)));
  }
  return str;
};

const TEST_CREDENTIALS = [
  { service: "TestKeytar", account: "TestASCII" },
  { service: "TestKeytar", account: "TestUTF8" },
  { service: "TestKeytar", account: "TestCharSet" },
  { service: "TestKeytar", account: "TestUTF16" },
  { service: "TestKeytar", account: "TestCJK" },
  { service: "TestKeytar", account: "TestBinary" },
  { service: "TestEmptyAccount", account: "" },
  { service: "", account: "TestEmptyService" },
  { service: "TestKeytar", account: "PwNullTerm" },
];

test.serial("get/setPassword with binary data", async (t) => {
  const binaryGroups =
    "01001000 01100101 01101100 01101100 01101111 00100000 01110111 01101111 01110010 01101100 01100100 00100001".match(
      /[01]{8}/g
    );
  const parsed = binaryGroups.map((binary) => parseInt(binary, 2));
  const buffer = Buffer.from(new Uint8Array(parsed).buffer);

  await setPassword("TestKeytar", "TestBinary", buffer.toString());
  const res = await getPassword("TestKeytar", "TestBinary");
  t.is(res, buffer.toString());
});

test.serial("get/setPassword with empty string parameters", async (t) => {
  // Empty "account" parameter
  await setPassword("TestEmptyAccount", "", "emptyAccountPW");
  const accountRes = await getPassword("TestEmptyAccount", "");
  t.is(accountRes, "emptyAccountPW");

  // Empty "service" parameter
  await setPassword("", "TestEmptyService", "emptyServicePW");
  const serviceRes = await getPassword("", "TestEmptyService");
  t.is(serviceRes, "emptyServicePW");
});

test.serial("get/setPassword with ASCII string", async (t) => {
  await setPassword("TestKeytar", "TestASCII", "ASCII string");

  const str = await getPassword("TestKeytar", "TestASCII");
  t.is(str, "ASCII string");
});

test.serial("get/setPassword with mixed character set", async (t) => {
  await setPassword("TestKeytar", "TestCharSet", "I 💔 ASCII");

  const str = await getPassword("TestKeytar", "TestCharSet");
  t.is(str, "I 💔 ASCII");
});

test.serial("get/setPassword with UTF-16 chars", async (t) => {
  await setPassword("TestKeytar", "TestUTF16", "🌞🌙🌟🌴");

  const str = await getPassword("TestKeytar", "TestUTF16");
  t.is(str, "🌞🌙🌟🌴");
});

test.serial("get/setPassword with UTF-8 chars", async (t) => {
  await setPassword(
    "TestKeytar",
    "TestUTF8",
    "ᚻᛖ ᚳᚹᚫᚦ ᚦᚫᛏ ᚻᛖ ᛒᚢᛞᛖ ᚩᚾ ᚦᚫᛗ ᛚᚪᚾᛞᛖ ᚾᚩᚱᚦᚹᛖᚪᚱᛞᚢᛗ ᚹᛁᚦ ᚦᚪ ᚹᛖᛥᚫ"
  );

  const str = await getPassword("TestKeytar", "TestUTF8");
  t.is(str, "ᚻᛖ ᚳᚹᚫᚦ ᚦᚫᛏ ᚻᛖ ᛒᚢᛞᛖ ᚩᚾ ᚦᚫᛗ ᛚᚪᚾᛞᛖ ᚾᚩᚱᚦᚹᛖᚪᚱᛞᚢᛗ ᚹᛁᚦ ᚦᚪ ᚹᛖᛥᚫ");
});

test.serial("get/setPassword with CJK symbols", async (t) => {
  await setPassword("TestKeytar", "TestCJK", "「こんにちは世界」");

  const str = await getPassword("TestKeytar", "TestCJK");
  t.is(str, "「こんにちは世界」");
});

test.serial("get/setPassword fails with null/undefined data", async (t) => {
  try {
    await setPassword("TestKeytar", "TestNull", null);
  } catch (err) {
    t.is(err.code, "StringExpected");
  }

  try {
    await setPassword("TestKeytar", "TestNull", undefined);
  } catch (err) {
    t.is(err.code, "StringExpected");
  }
});

test.serial(
  "get/setPassword with password containing extra null terminators",
  async (t) => {
    // "password" parameter w/ extra null terminator
    await setPassword("TestKeytar", "PwNullTerm", "PW\0");
    const pwRes = await getPassword("TestKeytar", "PwNullTerm");
    if (process.platform === "linux") {
      // libsecret automatically strips off null terminator
      t.is(pwRes, "PW");
    } else {
      t.is(pwRes, "PW\0");
    }
  }
);

test.serial("getPassword with missing data", async (t) => {
  const str = await getPassword("TestKeytar", "TestMissingPW");
  t.is(str, null);
});

test.serial(
  "findCredentials verifies that test credentials were stored",
  async (t) => {
    let expected = [
      { account: "TestASCII", password: "ASCII string" },
      { account: "TestBinary", password: "Hello world!" },
      { account: "TestCharSet", password: "I 💔 ASCII" },
      { account: "TestCJK", password: "「こんにちは世界」" },
      {
        account: "TestUTF8",
        password: "ᚻᛖ ᚳᚹᚫᚦ ᚦᚫᛏ ᚻᛖ ᛒᚢᛞᛖ ᚩᚾ ᚦᚫᛗ ᛚᚪᚾᛞᛖ ᚾᚩᚱᚦᚹᛖᚪᚱᛞᚢᛗ ᚹᛁᚦ ᚦᚪ ᚹᛖᛥᚫ",
      },
      { account: "TestUTF16", password: "🌞🌙🌟🌴" },
      { account: "PwNullTerm", password: "PW\x00" },
    ];
    const actual = await findCredentials("TestKeytar");
    t.is(actual.length, expected.length, `actual: ${JSON.stringify(actual)}; expected: ${JSON.stringify(expected)}`);

    expected.forEach((cred) =>
      t.not(
        actual.find((c) => c === cred),
        null
      )
    );
  }
);

test.serial("findPassword for ASCII string", async (t) => {
  const pw = await findPassword("TestKeytar/TestASCII");
  t.is(pw, "ASCII string");
});

test.serial("findPassword for mixed character set", async (t) => {
  const pw = await findPassword("TestKeytar/TestCharSet");
  t.is(pw, "I 💔 ASCII");
});

test.serial("findPassword for UTF-16", async (t) => {
  const pw = await findPassword("TestKeytar/TestUTF16");
  t.is(pw, "🌞🌙🌟🌴");
});

test.serial("findPassword for CJK symbols", async (t) => {
  const pw = await findPassword("TestKeytar/TestCJK");
  t.is(pw, "「こんにちは世界」");
});

test("deletePassword deletes all test credentials", async (t) => {
  console.log("\nThe deletePassword test is running. There is an intended delay of 5 seconds to wait for the keyring to update.");
  const timeout = (ms) => new Promise(resolve => setTimeout(resolve, ms));
  // initial timeout to give keyrings time to populate
  await timeout(5000);
  for (const cred of TEST_CREDENTIALS) {
    const result = await deletePassword(cred.service, cred.account);
    if (!result) {
      t.fail(`Credential with account "${cred.account}" failed to delete.`);
    }
  }

  const afterDeletion = await findCredentials("TestKeytar");
  t.is(afterDeletion.length, 0, `One or more credentials were still in the keyring: ${afterDeletion.map((c) => c.account).join(', ')}`);
});

// Unit tests specific to Windows API calls
if (process.platform === "win32") {
  test.serial(
    "setPassword fails when blob exceeds CRED_MAX_CREDENTIAL_BLOB_SIZE",
    async (t) => {
      console.log("win32: platform-specific tests for WinAPI");
      const CRED_MAX_CREDENTIAL_BLOB_SIZE = 5 * 512;
      const str = randomAsciiString(CRED_MAX_CREDENTIAL_BLOB_SIZE + 1);
      try {
        await setPassword("TestKeytarWindows", "MaxCredBlobSize", str);
      } catch (err) {
        t.not(err, null);
      }
    }
  );

  test.serial(
    "setPassword fails when TargetName exceeds CRED_MAX_GENERIC_TARGET_NAME_LENGTH",
    async (t) => {
      const CRED_MAX_GENERIC_TARGET_NAME_LENGTH = 32767;
      const str = randomAsciiString(CRED_MAX_GENERIC_TARGET_NAME_LENGTH + 1);
      try {
        await setPassword(
          "TestKeytarWindows",
          "MaxGenericTargetNameLen_".concat(str),
          "pw"
        );
      } catch (err) {
        t.not(err, null);
      }
    }
  );

  test.serial(
    "setPassword fails when account length exceeds CRED_MAX_USERNAME_LENGTH",
    async (t) => {
      const CRED_MAX_USERNAME_LENGTH = 512;
      const str = randomAsciiString(CRED_MAX_USERNAME_LENGTH + 1);
      try {
        await setPassword("TestKeytarWindows", str, "pw");
      } catch (err) {
        t.not(err, null);
      }
    }
  );

  test.serial(
    "findCredentials where CredEnumerateW returns false",
    async (t) => {
      const found = await findCredentials("TestKeytarWindowsInvalidService");
      t.deepEqual(found, []);
    }
  );

  test.serial("findCredentials where TargetName is NULL", async (t) => {
    // Since rust won't accept null as a parameter in the backend, best test is an empty string
    const found = await findCredentials("");
    t.is(found.length > 0, true);
  });

  test.serial(
    "Error handled when CredReadW throws ERROR_NOT_FOUND",
    async (t) => {
      try {
        const errorTest = await getPassword(
          "TestKeytarWindowsInvalidService",
          "FakeAccount"
        );
        t.is(errorTest, null);
      } catch (err) {
        t.fail(
          "getPassword should not throw an exception when no credentials are found (win32)"
        );
      }
    }
  );

  test.serial(
    "CredDeleteW with a credential that does not exist",
    async (t) => {
      try {
        await deletePassword("TestKeytarWindowsInvalidService", "FakeAccount");
      } catch (err) {
        t.fail(
          "deletePassword should not throw an exception for a credential that doesn't exist (win32)"
        );
      }

      t.pass();
    }
  );
}
