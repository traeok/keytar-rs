import test from "ava";
import {
  deletePassword,
  findCredentials,
  findPassword,
  getPassword,
  setPassword,
} from "../index.js";

test.only("get/setPassword with ASCII string", async (t) => {
  await setPassword("TestKeytar", "TestASCII", "ASCII string");

  const str = await getPassword("TestKeytar", "TestASCII");
  t.is(str, "ASCII string");
});

test.only("get/setPassword with mixed character set", async (t) => {
  await setPassword("TestKeytar", "TestCharSet", "I 💔 ASCII");

  const str = await getPassword("TestKeytar", "TestCharSet");
  t.is(str, "I 💔 ASCII");
});

test.only("get/setPassword with UTF-16 chars", async (t) => {
  await setPassword("TestKeytar", "TestUTF16", "🌞🌙🌟🌴");

  const str = await getPassword("TestKeytar", "TestUTF16");
  t.is(str, "🌞🌙🌟🌴");
});

test.only("get/setPassword with CJK symbols", async (t) => {
  await setPassword("TestKeytar", "TestCJK", "「こんにちは世界」");

  const str = await getPassword("TestKeytar", "TestCJK");
  t.is(str, "「こんにちは世界」");
});

test("findCredentials prints credentials in test service and verifies member names", async (t) => {
  const pws = await findCredentials("TestKeytar");
  t.is(pws.length, 4);
  console.log(pws);
  t.is(pws[0].hasOwnProperty("account"), true);
  t.is(pws[0].hasOwnProperty("password"), true);
});

test("findPassword for ASCII string", async (t) => {
  const pw = await findPassword("TestKeytar/TestASCII");
  t.is(pw, "ASCII string");
});

test("findPassword for mixed character set", async (t) => {
  const pw = await findPassword("TestKeytar/TestCharSet");
  t.is(pw, "I 💔 ASCII");
});

test("findPassword for UTF-16", async (t) => {
  const pw = await findPassword("TestKeytar/TestUTF16");
  t.is(pw, "🌞🌙🌟🌴");
});

test("findPassword for CJK symbols", async (t) => {
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
