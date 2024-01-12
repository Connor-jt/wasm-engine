// script intended to be run in web console for website: http://ref.x86asm.net/coder64.html

// ## DONE ##
// generate opcode to struct index function

// ## TODO ##
// generate opcode struct list  ?? (how the hell do you do that in rust??)
// generate prefix list

// ## NOTES ##
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

    object.has_r = false
    object.has_rm = false
    object.full_rm = false
    return object
}
// output
let instruction_list = []

const r8_regs = ["AL", "CL", "DL", "BL", "AH", "CH", "DH", "BH"]
const r16_32_regs = ["eAX", "eCX", "eDX", "eBX", "eSP", "eBP", "eSI", "eDI"]


function run(){
    let testing_index = 0
    for (var i = 0; i < document.body.childNodes.length; i++) {
        let item = document.body.childNodes[i]
        if (item.nodeName == "TABLE"){
            let is_0f = (testing_index > 0) // next table has 0f for the first byte instead
            testing_index += 1
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
                        if (width != null) {
                            column_index += Number(width)
                            console.log(table_item)
                        }
                        else column_index += 1

                        // unwrap thing
                        while (value.indexOf('<') > -1 && (col == "op1" || col == "op2" || col == "op3" || col == "op4") ){
                            row_item = row_item.childNodes[0]
                            let extended_text = row_item.getAttribute("title")
                            if (row_item.nodeName == "SPAN"){
                                // then we have to abort this one?
                                instruction.aborted = row_item.innerHTML // we want to keep the stuff as a note or something
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
                            case "o":
                                if (value != null) instruction.has_rm = true // we have to track whether the rm is used, so we can properly count bytes or something
                                if (value != "r") instruction.rm = value; 
                                else instruction.has_r = true
                                break
                            case "mnemonic": 
                                //if (instruction.b1 != null && instruction.b1.indexOf("+r") != -1) console.log("found a working one")
                                if (value != "invalid" && value != "<i>invalid</i>" &&  value != "no mnemonic" && value != "undefined" && value != null) instruction.name = filter_name(value); 
                                else instruction.aborted = value; 
                                break
                            case "op1": instruction.op1 = filter_op(value); break
                            case "op2": instruction.op2 = filter_op(value); break
                            case "op3": instruction.op3 = filter_op(value); break
                            case "op4": instruction.op4 = filter_op(value); break
                            case "notes": instruction.note = value; break
                    }}
                    // check if this is a prefix or a instruction
                    if (instruction.pre != null && instruction.b1 == null)
                         prefixes.push(instruction)
                    else {
                        // if the opcodes have +r in them, if so then we have to split the instruction up into each respective register version of that opcode
                        // we then replace the first parameter with either the 16/32 bit or 8 bit register
                        let is_second_op_byte = (instruction.b2 != null)
                        let target_byte;
                        if (is_second_op_byte) target_byte = instruction.b2
                        else target_byte = instruction.b1

                        if (target_byte.indexOf("+r") > -1){
                            
                            // clear values that we do not need
                            let opcode_number = parseInt(target_byte.slice(0, -2), 16)

                            let regs_array;
                            if (instruction.op1 == "r8") regs_array = r8_regs
                            else regs_array = r16_32_regs
                            for (let reg_index = 0; reg_index < regs_array.length; reg_index++){
                                // clone
                                let new_instruction = {}
                                for (var attr in instruction) new_instruction[attr] = instruction[attr];
                                // update opcode byte
                                let updated_opcode_byte = (opcode_number+reg_index).toString(16);
                                if (is_second_op_byte) new_instruction.b2 = updated_opcode_byte
                                else new_instruction.b1 = updated_opcode_byte   
                                // update param[0] register
                                new_instruction.op1 = regs_array[reg_index]
                                // then push
                                instructions.push(new_instruction)
                            }
                        } 
                        else instructions.push(instruction)
                    }


            }}

    }}
    // list info

    // list unique instruction names
    let func_dict = {}
    instructions.forEach(element => {func_dict[element.name] = true})

    // list unique operand types
    let op_dict = {} // <operand, list_index>
    let op_list = []
    instructions.forEach(e => {
        add_op(op_dict, op_list, e.op1)
        add_op(op_dict, op_list, e.op2)
        add_op(op_dict, op_list, e.op3)
        add_op(op_dict, op_list, e.op4)})

    let operands_code = "enum operand{\n"
    op_list.forEach((element, index) => {
        operands_code += "   " + element + " = " + index + ",\n"
    });
    operands_code += "}"

    let errored_instructions = []
    // start compiling the instruction tree
    let top_level = {}
    instructions.forEach(e => {
        if (e.b1 == null) {
            console.log(e)
            throw "bad element, has no byte1"

        }
        if (e.pre != null || e.aborted != false){
            errored_instructions.push(e)
            return // skip iteration as this one creates problems with our current system
        }
        // god bless intel for this holy mess that i've had to create to make this work !!!
        let index = instruction_list.length
        instruction_list.push(e)
        // sort into layer 1
        if (!(e.b1 in top_level)) top_level[e.b1] = {instruction:index}
        // sort into layer 2
        let layer2 = top_level[e.b1]
        if (e.b2 != null){
            if (e.b3 == null && instruction_list[layer2.instruction].rm != null && layer2.instruction != index){
                let l3_target = "rm_" + e.rm
                if (!(l3_target in layer2)) layer2[l3_target] = {instruction:-1} // if not entered already, then this is a dumb override
                let layer3 = layer2[l3_target]

                if (!(e.b2 in layer3)) layer3[e.b2] = {instruction:index}
                else throw "how is something already using the override b2"
            } else {
                if (!(e.b2 in layer2)) layer2[e.b2] = {instruction:index}
                let layer3 = layer2[e.b2]
                // sort into layer 3
                if (e.b3 != null){
                    if (instruction_list[layer3.instruction].rm != null){
                        let l4_target = "rm_" + e.rm
                        if (!(l4_target in layer3)) layer3[l4_target] = {instruction:-1}
                        let layer4 = layer3[l4_target]

                        if (!(e.b3 in layer4)) layer4[e.b3] = {instruction:index}
                        else throw "how is something already using the override b3"
                    }
                    else if (!(e.b3 in layer3)) layer3[e.b3] = {instruction:index}
                    // there is no layer 4, so the rm byte past this point does not matter
                } else try_append_rm_byte(layer3, e, index)
        }} else try_append_rm_byte(layer2, e, index)
    });
    // now convert the tree into code
    let instruction_reader_code = "fn get_instruction_index(byte1:u8, byte2:u8, byte3:u8) -> Option<u32>{\n"
    instruction_reader_code += "   " + recurse_instruction_tree(top_level, 1) + "}"

    // compile the prefix tree



    // compile instructions list
    let instructions_list_code = "lazy_static!{static ref INSTRUCTIONS:Vec<instruction> = vec![\n"
    for (let i = 0; i < instruction_list.length; i++) {
        let item = instruction_list[i]
        // we cant skip any things
        //if (item.b1 == "0F") continue // dont write this one because it makes a mess + isn't an opcode
        if (item.name[0] == '<') item.name = "_0F" // better fix?

        instructions_list_code += "    instruction{name:\""
        instructions_list_code += item.name
        instructions_list_code += "\".to_owned(), params:vec!["
        // we'd have to update to a list system for this java-script, so we can support the rust list system
        if (item.op1 != null) instructions_list_code += "operand::" + item.op1 + ","
        if (item.op2 != null) instructions_list_code += "operand::" + item.op2 + ","
        if (item.op3 != null) instructions_list_code += "operand::" + item.op3 + ","
        if (item.op4 != null) instructions_list_code += "operand::" + item.op4 + ","
        instructions_list_code += "], rm_byte:rm_type::"
        if (item.has_r) instructions_list_code += "available"
        else if (item.has_rm == false) instructions_list_code += "none"
        else if (item.full_rm == false) instructions_list_code += "reg_opcode"
        else instructions_list_code += "full_opcode"
        // figure out the opcode length
        let byte_count = 1
        if (item.b2 != null) byte_count += 1
        if (item.b3 != null) byte_count += 1
        if (item.has_r) byte_count += 1
        instructions_list_code += ", opc_length:" + byte_count

        instructions_list_code += ", opc1:"
        instructions_list_code += "0x" + item.b1
        instructions_list_code += ", opc2:"
        if (item.b2 != null )instructions_list_code += "Some(0x" + item.b2 + ")"
        else instructions_list_code += "None"
        instructions_list_code += ", opc3:"
        if (item.b3 != null )instructions_list_code += "Some(0x" + item.b3 + ")"
        else instructions_list_code += "None"
        instructions_list_code += ", opc4_reg:"
        if (item.rm != null )instructions_list_code += "Some(0x" + item.rm + ")"
        else instructions_list_code += "None"
        instructions_list_code += "},\n"
    }
    instructions_list_code += "];}"
    //
    let prefixes_list = ""
    for (let i = 0; i < prefixes.length; i++){
        let curr_prefix = prefixes[i]
        //console.log(curr_prefix)
        prefixes_list += "0x" + curr_prefix.pre + ": " + curr_prefix.name + " //" + curr_prefix.note +"\n"
    }

    // OUT REGULAR INFO //
    console.log(operands_code)
    console.log(instruction_reader_code)
    console.log(instructions_list_code)
    //console.log(prefixes_list)

    // DEBUG INFO //
    console.log("instructions: " + instructions.length)
    console.log("unqiue instructions: " + Object.keys(func_dict).length)
    console.log("total printed instructions: " + instruction_list.length)
    console.log("prefixes: " + prefixes.length)
    console.log("unique operands: " + Object.keys(op_dict).length)
    console.log("rejected instructions: " + errored_instructions.length)
}
function recurse_instruction_tree(layer, depth){
    let content = ""
    let is_rm = null
    // vars to track if we are using the 
    let is_fallback = false
    if (layer.instruction != null ){ // so we dont try to get instruction of top level element
        is_fallback = true
        // check if this instruction is used by any child layers
        if (layer.instruction != -1) {
            for (let key in layer){
                let element = layer[key]
                if (element.instruction == layer.instruction){
                    is_fallback = false
                    break
                }
            }
            if (instruction_list[layer.instruction].rm == null)
                is_fallback = false // screw it, we just run the loop as well for 0F, but uncheck is fallback after
        }
    } //else is_fallback = true
    for (let key in layer){
        if (key == "instruction") continue; // we want to skip the element assigned as the instruction index
        
        // check whether this is rm or not
        let rm_state = (key.indexOf('rm_') > -1)
        if (is_rm == null) is_rm = rm_state
        else if (rm_state != is_rm) throw "rm states did not match"
        
        if (is_rm) content += "   ".repeat(depth+1) + "0x" + key.slice(3) + " => {"
        else       content += "   ".repeat(depth+1) + "0x" + key + " => {"

        let element = layer[key]
        if (Object.keys(element).length > 1){ // this layer has a lower layer
            if (is_fallback) {
                console.log(key)
                console.log(element)
                console.log(layer)
                console.log(instruction_list[layer.instruction])
                throw "fallback instruction cannot contain more instructions"
            }
            
            content += recurse_instruction_tree(element, depth+1)
        }else{
            content += "return Some(" + element.instruction + ");"
            if (is_fallback){
                instruction_list[element.instruction].full_rm = true
                instruction_list[element.instruction].rm = null
                //content += "// uses whole rm byte for opcode"
            }
        }
        content += "}\n"
    }
    let header = "" //"   ".repeat(depth);
    let ender = ""
    if (is_fallback) {
        depth -= 1 // fallbacks switch case on the same byte (as the upper layer did match (value & 0b00111000))
        if (layer.instruction != -1)
             ender += "_ => {return Some("+ layer.instruction +")}}"
        else ender += "_ => {return None}}" // if no fallback for override, then we have to return none as the default
    }else ender += "_ => {return None}}"
    
    if (is_rm) header += "match r_bits!(byte"+ depth +"){ // RM 3bit value as opcode\n"
    else       header += "match byte"+ depth +"{\n"

    if (is_fallback) depth += 1 // fixup the depth
    return header + content + "   ".repeat(depth+1) + ender
}

function try_append_rm_byte(layer, element, index){
    if (element.rm != null){
        let string = "rm_" + element.rm
        
        if (string in layer){
            return // usually the first occurance seems like the better one?
            console.log("rm " + element.b1 + ","+ element.b2 +","+ element.b3+":"+element.rm+ " overwritten") 
            //throw "this rm already exists??"
        }
        layer[string] = {instruction:index}
    }
}
function filter_name(name){
    return name.replace("<i>", "").replace("</i>", "").replace("\n", "")
}
let digit_pattern = /\d/; 
function filter_op(operand){
    if (operand == "m16/32&amp;16/32") return "m16_32" // not really sure what this one means
    if (operand[0].match(digit_pattern)) operand = "_" + operand
    return operand.replace('/', '_').replace('/', '_').replace('/', '_').replace(':', '_').replace(' ', '_') // yep, we have to double up on the slash check
}
function add_op(dict, list, operand){
    if (operand == null) return
    if (!(operand in dict)) {
        dict[operand] = list.length
        list.push(operand)
}}
run()