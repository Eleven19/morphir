import { processJobWithCallback, submitJob } from '../index';


processJobWithCallback("Hello, Elm!", (data: any) => {
    console.log(data);
});