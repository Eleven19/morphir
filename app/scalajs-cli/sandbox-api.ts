import { Elm } from './elm/projects/sandbox/src/Main.elm';

let main = Elm.Main.init();


function submitJob(input: string) {
    main.ports.get.send(input);
}

function processJobAsync(input: string) {
    return new Promise((resolve, reject) => {
        main.ports.get.send(input);
        main.ports.put.subscribe(resolve);
    });
}

function processJobWithCallback(input: string, callback: (data: any) => void) {
    main.ports.get.send(input);
    main.ports.put.subscribe(callback);
}

exports.submitJob = submitJob;
exports.processJobAsync = processJobAsync;
exports.processJobWithCallback = processJobWithCallback;

// export function subscribeToResults(callback: (data: any) => void) {
//     main.ports.put.subscribe(callback);
// }