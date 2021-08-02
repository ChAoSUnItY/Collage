import { import_wasm } from './import_wasm';

(async () => {
    const collage = await import_wasm();

    let result = collage.parse_as_json("a <- 1");

    console.log(result);
})()