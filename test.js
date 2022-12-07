const keytarNode = require("./node_modules/keytar");
const keytarRust = require("./keytar.node");

(async () => {
    await keytarNode.setPassword("test", "user1", "password");
    console.log("NN:", await keytarNode.getPassword("test", "user1"));
    console.log("NR:", await keytarRust.getPassword("test", "user1"));
    await keytarRust.setPassword("test", "user2", "password");
    console.log("RN:", await keytarNode.getPassword("test", "user2"));
    console.log("RR:", await keytarRust.getPassword("test", "user2"));

    await keytarNode.setPassword("test", "user3", "💖🦀");
    console.log("NN:", await keytarNode.getPassword("test", "user3"));
    console.log("NR:", await keytarRust.getPassword("test", "user3"));
    await keytarRust.setPassword("test", "user4", "💖🦀");
    console.log("RN:", await keytarNode.getPassword("test", "user4"));
    console.log("RR:", await keytarRust.getPassword("test", "user4"));

    await keytarNode.setPassword("test", "user5", "password💖");
    console.log("NN:", await keytarNode.getPassword("test", "user5"));
    console.log("NR:", await keytarRust.getPassword("test", "user5"));
    await keytarRust.setPassword("test", "user6", "password💖");
    console.log("RN:", await keytarNode.getPassword("test", "user6"));
    console.log("RR:", await keytarRust.getPassword("test", "user6"));
})();
