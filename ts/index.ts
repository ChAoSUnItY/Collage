import $ from 'jquery';
import { import_wasm } from './import_wasm';

(async () => {
    const collage = await import_wasm();

    $("#source").bind("input propertychange", () => {
        let source = $("#source").val()?.toString();

        if (source != undefined) {
            console.log(typeof source)
            console.log(source)

            let result = collage.parse_as_json(source);

            console.log(result);

            $("#structure").text(JSON.stringify(result, null, ""));

            console.log(JSON.stringify(result, null, ""));
            console.log($("#structure").text())
        }
    })
})()