import { Elm } from './src/Main.elm';

let main = Elm.Main.init();


export function submitJob(input: string) {
    main.ports.get.send(input);
}

export function processJobAsync(input: string) {
    return new Promise((resolve, reject) => {
        main.ports.get.send(input);
        main.ports.put.subscribe(resolve);
    });
}

export function processJobWithCallback(input: string, callback: (data: any) => void) {
    main.ports.get.send(input);
    main.ports.put.subscribe(callback);
}

export default {
    submitJob,
    processJobAsync,
    processJobWithCallback
};

// export function subscribeToResults(callback: (data: any) => void) {
//     main.ports.put.subscribe(callback);
// }