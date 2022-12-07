const keytarNode = require("../zowe/zowe-cli/node_modules/keytar");
const keytarRust = require("./keytar.node");

(async () => {
    await keytarNode.setPassword("test", "user1", "password");
    console.log(await keytarNode.getPassword("test", "user1"));
    console.log(keytarRust.getPassword("test", "user1"));
    await keytarRust.setPassword("test", "user2", "password");
    console.log(await keytarNode.getPassword("test", "user2"));
    console.log(keytarRust.getPassword("test", "user2"));
    await keytarNode.setPassword("test", "user3", "password💖");
    console.log(await keytarNode.getPassword("test", "user3"));
    console.log(keytarRust.getPassword("test", "user3"));
    await keytarRust.setPassword("test", "user4", "password💖");
    console.log(await keytarNode.getPassword("test", "user4"));
    console.log(keytarRust.getPassword("test", "user4"));
})();
