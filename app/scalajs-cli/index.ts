import { processJobWithCallback, submitJob } from '@finos/mock-morphir-elm';


processJobWithCallback("Hello, Elm!", (data: any) => {
    console.log(data);
});