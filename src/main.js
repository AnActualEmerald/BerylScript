const compiler = require("./compiler.js");
const fs = require("fs");

function loadfile(path) {
    let res = "";
    let data = fs.readFileSync(path)
    
    res = data.toString('utf-8');
    
    return res;

}

function main() {
    let path = process.argv[2];
    // console.log(process.argv[2]);
    let f = loadfile(path);
    let tokens = compiler.lex(f);
    console.log(tokens);
}

main();

