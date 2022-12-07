const keytarNode = require("./node_modules/keytar");
const keytarRust = require("./keytar.node");

(async () => {
    await keytarNode.setPassword("test", "user1", "password");
    console.log(await keytarNode.getPassword("test", "user1"));
    console.log(await keytarRust.getPassword("test", "user1"));
    await keytarRust.setPassword("test", "user2", "password");
    console.log(await keytarNode.getPassword("test", "user2"));
    console.log(await keytarRust.getPassword("test", "user2"));

    await keytarNode.setPassword("test", "user3", "💖💖");
    console.log(await keytarNode.getPassword("test", "user3"));
    console.log(await keytarRust.getPassword("test", "user3"));
    await keytarRust.setPassword("test", "user4", "💖💖");
    console.log(await keytarNode.getPassword("test", "user4"));
    console.log(await keytarRust.getPassword("test", "user4"));

    await keytarNode.setPassword("test", "user5", "password💖");
    console.log(await keytarNode.getPassword("test", "user5"));
    console.log(await keytarRust.getPassword("test", "user5"));
    await keytarRust.setPassword("test", "user6", "password💖");
    console.log(await keytarNode.getPassword("test", "user6"));
    console.log(await keytarRust.getPassword("test", "user6"));
})();
