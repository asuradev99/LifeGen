import WasmInit, {start} from "./pkg/lifegen.js";
const rustWasm = await WasmInit("./pkg/lifegen_bg.wasm");


start();