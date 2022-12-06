const keytar = require("./keytar.node");
// console.log(Buffer.from(keytar.getPassword("Zowe", "secure_config_props"), "utf16le").toString());
console.log(keytar.getPassword("Zowe", "secure_config_props"));
// keytar.setPassword("Zowe", "test", "hello");
