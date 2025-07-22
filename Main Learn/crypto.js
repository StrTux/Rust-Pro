const crypto = require("crypto");

function findHashWithPrefix(prefix) {
    let input = 569193;
    while (true) {
        const inputStr = `100xdevs${input}`;
        const hash = crypto.createHash("sha256").update(inputStr).digest("hex");
        if (hash.startsWith(prefix)) {
            return { input, hash };
        }
        input++;
    }
}

const result = findHashWithPrefix("00000");
console.log(`Input: ${result.input}`);
console.log(`Hash: ${result.hash}`);
