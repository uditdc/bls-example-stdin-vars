import { memory } from "@blockless/sdk";

// Get standard stdin
const stdin = new memory.Stdin().read();
const stdinString = stdin.toString().replaceAll("\0", "").trim();

console.log('stdinString: ' + stdinString)
