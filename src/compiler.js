const validChars = "abcdefghijklmnopqrstuvwxyz";
const validNums = "0123456789";
const validSymb = "{}_()=;\"";

const state = {
    nothing: "n",
    EmString: "es",
    EmName: "en",
    EmNumber: "enum"
}

function lex(data) {
    let result = [];
    let tok = [];
    let curState = state.nothing;

    for(i = 0; i < data.length; i++){
            let c = data[i];

            // console.log(c);
            if(validChars.includes(c.toLowerCase())) {
                // console.log("char");
                if(curState === state.nothing){
                    curState = state.EmName;
                    tok.push(c);
                } else if(curState === state.EmName || curState === state.EmString){
                    tok.push(c);
                }
            }

            else if(validNums.includes(c)) {
                // console.log("num");
                if(curState === state.nothing){
                    curState = state.EmNumber;
                    tok.push(c);
                }else if (curState === state.EmNumber){
                    tok.push(c);
                }
            }

            else if(validSymb.includes(c)) {
                // console.log("symbs");

                if(c === '\"' && curState !== state.EmString){
                    curState = state.EmString;
                    continue;
                }else if (c==='\"' && curState === state.EmString){
                    curState = state.nothing;
                    result.push({
                        name: "string",
                        value: tok.join("")
                    });
                    tok = [];
                    continue;
                }
                if(curState === state.nothing){
                    result.push({
                        name: "op",
                        value: c
                    });
                }else if (curState === state.EmName) {
                    result.push({
                        name: "name",
                        value: tok.join("")
                    });
                    tok = [];
                    curState = state.nothing;
                    result.push({
                        name: "op",
                        value: c
                    });
                }
                else if (curState === state.EmNumber) {
                    result.push({
                        name: "number",
                        value: tok.join("")
                    });
                    tok = [];
                    curState = state.nothing;
                    result.push({
                        name: "op",
                        value: c
                    });
                }

                if(c === ';') {
                    result.pop();
                    result.push({
                        name: "eol",
                        value: c
                    });
                }
            }

            else {
                // console.log("default");
                if(curState === state.EmName){
                    result.push({
                        name: "name",
                        value: tok.join("")
                    });
                    tok = [];
                    curState = state.nothing;
                } else if (curState === state.EmNumber){
                    result.push({
                        name: "number",
                        value: tok.join("")
                    });
                    tok = [];
                    curState = state.nothing;
                } else if (curState === state.EmString){
                    tok.push(" ");
                }
            }
    }


    return result;
}

function treeGen(tokens){

}

function compile(tokens){
    let ast = {};

}

module.exports = {
    lex: lex,
    compile: compile
}