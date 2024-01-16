






def run():
    file = open("types.txt")
    for line in file:
        # if line either starts with '/' or is empty then we skip
        if len(line) == 0 or line[0] == '/' or line[0] == '\n':
            continue
        op_type = line.split(" = ")[0]
        print("                asm86::operand::" + op_type + "=>{operand_string=\""+ op_type +"\".to_owned()},") 

run()