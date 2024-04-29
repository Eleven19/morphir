import { Elm } from './elm/projects/sandbox/src/Main.elm';

let main = Elm.Main.init();
// Get data from the command line
var args = process.argv.slice(2);
var input = args[0];
console.log("\n   Input: ", input)

// Send data to the worker
main.ports.get.send(input);
main.ports.get.send(input + "2");
main.ports.get.send(input + "3");

main.ports.put.subscribe((data) => {
    console.log("   Output: " + JSON.stringify(data) + "\n");
});

