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

            $("#structure").html(syntaxHighlight(JSON.stringify(result, null, 2)));
        }
    })
})()

function syntaxHighlight(json: string) {
    if (typeof json != 'string') {
         json = JSON.stringify(json, undefined, 2);
    }
    json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    return json.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
        var cls = 'number';
        if (/^"/.test(match)) {
            if (/:$/.test(match)) {
                cls = 'key';
            } else {
                cls = 'string';
            }
        } else if (/true|false/.test(match)) {
            cls = 'boolean';
        } else if (/null/.test(match)) {
            cls = 'null';
        }
        return '<span class="' + cls + '">' + match + '</span>';
    });
}