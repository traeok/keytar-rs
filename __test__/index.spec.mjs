import test from "ava";
import {
  deletePassword,
  findCredentials,
  findPassword,
  getPassword,
  setPassword,
} from "../index.js";

test.only("can use get/setPassword with ASCII string", async (t) => {
  const result = await setPassword("TestKeytar", "TestASCII", "ASCII string");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestASCII");
  t.is(str, "ASCII string");
});

test.only("can use get/setPassword with mixed character set", async (t) => {
  const result = await setPassword("TestKeytar", "TestCharSet", "I 💔 ASCII");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestCharSet");
  t.is(str, "I 💔 ASCII");
});

test.only("can use get/setPassword with UTF-16 chars", async (t) => {
  const result = await setPassword("TestKeytar", "TestUTF16", "🌞🌙🌟🌴");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestUTF16");
  t.is(str, "🌞🌙🌟🌴");
});

test.only("can use get/setPassword with CJK symbols", async (t) => {
  const result = await setPassword(
    "TestKeytar",
    "TestCJK",
    "「こんにちは世界」"
  );
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestCJK");
  t.is(str, "「こんにちは世界」");
});

test("findCredentials prints out all credentials in test service", async (t) => {
  const pws = await findCredentials("TestKeytar");
  t.is(pws.length, 4);
  console.log(pws);
});

test("findPassword works for ASCII test", async (t) => {
  const pw = await findPassword("TestKeytar/TestASCII");
  t.is(pw, "ASCII string");
});

test("findPassword works for mixed character set", async (t) => {
  const pw = await findPassword("TestKeytar/TestCharSet");
  t.is(pw, "I 💔 ASCII");
});

test("findPassword works for UTF-16 test", async (t) => {
  const pw = await findPassword("TestKeytar/TestUTF16");
  t.is(pw, "🌞🌙🌟🌴");
});

test("findPassword works for CJK symbols test", async (t) => {
  const pw = await findPassword("TestKeytar/TestCJK");
  t.is(pw, "「こんにちは世界」");
});

test("deletePassword deletes all test credentials", async (t) => {
  const ascii = await deletePassword("TestKeytar", "TestASCII");
  t.is(ascii, true);
  const test_charset = await deletePassword("TestKeytar", "TestCharSet");
  t.is(test_charset, true);
  const utf16 = await deletePassword("TestKeytar", "TestUTF16");
  t.is(utf16, true);
  const cjk = await deletePassword("TestKeytar", "TestCJK");
  t.is(cjk, true);
});
