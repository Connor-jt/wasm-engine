// script intended to be run in web console for website: http://ref.x86asm.net/coder64.html


// generate opcode struct list
// generate opcode to struct index function
// generate prefix list

// TODO //
// row_item wrapped in italic means its implicit? bold means its a dest
// we have to write implicits into comments, and only have explicits be passed into functions or whatever


let fields = ["pf","0f","po","so","o","proc","st","m","rl","x","mnemonic","op1","op2","op3","op4","iext","tested f","modif f","def f","undef f","f values","notes"]

let prefixes = []
let instructions = []
let instruction_tree = {} // <name, index>
function generate_struct(){
    let object = {}
    object.pre = null
    object.b1 = null
    object.b2 = null
    object.b3 = null
    object.rm = null
    object.op1 = null
    object.op2 = null
    object.op3 = null
    object.op4 = null
    object.name = null
    object.note = null
    object.aborted = false
    return object
}

let known_operands = {}

function run(){

    for (var i = 0; i < document.body.childNodes.length; i++) {
        let item = document.body.childNodes[i]
        if (item.nodeName == "TABLE"){
            let is_0f = i > 0 // next table has 0f for the first byte instead

            for (var j = 0; j < item.childNodes.length; j++) {
                let table_item = item.childNodes[j]
                if (table_item.nodeName == "TBODY"){
                    let row = table_item.childNodes[0]
                    while (row.nodeName != "TR") { // skip any text elements
                        if (row.nextElementSibling == null) throw "assert: row has no valid entry"
                        row = row.nextElementSibling
                    }

                    let instruction = generate_struct()
                    let column_index = 0
                    for (var k = 0; k < row.childNodes.length; k++) {
                        let row_item = row.childNodes[k]
                        if (row_item.nodeName != "TD") continue
                        let col = fields[column_index]
                        let value = row_item.innerHTML

                        // increment column index
                        //console.log(row_item)
                        let width = row_item.getAttribute("colspan")
                        if (width != null) column_index += width
                        else               column_index += 1

                        // unwrap thing
                        while (value.indexOf('<') > -1 && (col == "op1" || col == "op2" || col == "op3" || col == "op4") ){
                            row_item = row_item.childNodes[0]
                            let extended_text = row_item.getAttribute("title")
                            if (row_item.nodeName == "SPAN"){
                                // then we have to abort this one?
                                instruction.aborted = true
                                value = ""
                                break
                            }
                            if (extended_text != null){
                                value = extended_text
                                break}
                            value = row_item.innerHTML
                        }
                        
                        if (value == "") continue // if column is empty then we dont need to write value

                        switch(col){
                            case "pf": instruction.pre = value; break
                            case "0f": if (is_0f) instruction.b1 = value; break
                            case "po":
                                if (is_0f) instruction.b2 = value
                                else       instruction.b1 = value
                                break
                            case "so":
                                if (is_0f) instruction.b3 = value
                                else       instruction.b2 = value
                                break
                            case "o": if (value != "r") instruction.pre = value; break
                            case "mnemonic": if (value != "invalid" && value != "no mnemonic") instruction.name = value; break
                            case "op1": instruction.op1 = value; break
                            case "op2": instruction.op2 = value; break
                            case "op3": instruction.op3 = value; break
                            case "op4": instruction.op4 = value; break
                            case "notes": instruction.note = value; break
                    }}
                    // check if this is a prefix or a instruction
                    if (instruction.pre != null && instruction.b1 == null)
                         prefixes.push(instruction)
                    else instructions.push(instruction)


            }}

    }}
    
    // list unique instruction names
    let func_dict = {}
    instructions.forEach(element => {func_dict[element.name] = true})
    console.log("unqiue instructions: " + Object.keys(func_dict).length)

    // list unique operand types
    let op_dict = {} // <operand, list_index>
    let op_list = []
    instructions.forEach(e => {
        add_op(op_dict, op_list, e.op1)
        add_op(op_dict, op_list, e.op2)
        add_op(op_dict, op_list, e.op3)
        add_op(op_dict, op_list, e.op4)})
    console.log("unique operands: " + Object.keys(op_dict).length)

    let operands_code = "enum operands{\n"
    op_list.forEach((element, index) => {
        operands_code += "   " + element + " = " + index + ",\n"
    });
    operands_code += "}"
    console.log(operands_code)


}
function add_op(dict, list, operand){
    if (operand == null) return
    if (!(operand in dict)) {
        dict[operand] = list.length
        list.push(operand)
}}
run()