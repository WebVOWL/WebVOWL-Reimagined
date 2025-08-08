// import { threads } from 'wasm-feature-detect';
// import * as Comlink from "comlink";
// (async function initMultiThreads() {
//     if (!(await threads())) return;
//     const multiThread = await import("../../rust/pkg/index.js");
//     await multiThread.default();
//     await multiThread.initThreadPool(navigator.hardwareConcurrency);
//     testProg();
// })();
import init, { initThreadPool, run } from "../target/pkg/index";

// Regular wasm-bindgen initialization.
init().then(
    // Thread pool initialization with the given number of threads
    // (pass `navigator.hardwareConcurrency` if you want to use all cores).
    await initThreadPool(navigator.hardwareConcurrency).then(testProg()),
);

// Wasm workers: https://github.com/sgasse/wasm_worker_interaction
// Simplify web workers: https://github.com/GoogleChromeLabs/comlink

function testProg() {
    console.log("Starting");
    let input = document.createElement("input");
    input.type = "file";
    // const worker = new Worker("./worker.js");
    // const obj = Comlink.wrap(worker)
    input.onchange = (e) => {
        let file = e.target.files[0];
        run(file);
        console.log("Code run");
    };
    document.appendChild(input);
}
