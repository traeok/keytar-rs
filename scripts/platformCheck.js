const fs = require("fs");
const path = require("path");

function getTargetName() {
    switch (process.platform) {
        case "linux":
            const isMusl = process.report.getReport().header.glibcVersionRuntime == null;
            switch (process.arch) {
                case "arm":
                    return "linux-arm-gnueabihf";
                case "arm64":
                    return `linux-arm64-${isMusl ? "musl" : "gnu"}`;
                case "ia32":
                    return "linux-ia32-gnu";
                case "x64":
                    return `linux-x64-${isMusl ? "musl" : "gnu"}`;
            }
        case "win32":
            return `win32-${process.arch}-msvc`;
        case "darwin":
        default:
            return `${process.platform}-${process.arch}`;
    }
}

const prebuildPath = path.join("prebuilds", `keytar-rs.${getTargetName()}.node`);
fs.accessSync(prebuildPath);
