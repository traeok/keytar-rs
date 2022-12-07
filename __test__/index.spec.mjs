import test from "ava";
import { getPassword, setPassword } from "../index.js";

test("can use get/setPassword on ASCII string", async (t) => {
  const result = await setPassword("TestKeytar", "TestASCII", "ASCII string");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestASCII");
  t.is(str, "ASCII string");
});

test("can use get/setPassword on mixed character set", async (t) => {
  const result = await setPassword("TestKeytar", "TestCharSet", "I 💔 ASCII");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestCharSet");
  t.is(str, "I 💔 ASCII");
});

test("can use get/setPassword on UTF-16 chars", async (t) => {
  const result = await setPassword("TestKeytar", "TestUTF16", "🌞🌙🌟🌴");
  t.is(result, true);

  const str = await getPassword("TestKeytar", "TestUTF16");
  t.is(str, "🌞🌙🌟🌴");
});
