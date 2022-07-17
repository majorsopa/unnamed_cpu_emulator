pub enum Instruction {
    PUSH,       // pushes to stack
    POP,        // pops from stack
    JUMP,       // jumps to address
    CALL,       // calls address
    RET,        // returns from call
    CMP,        // compares two values, setting a flag if they're equal or not
    JEQ,        // jumps to address if flag is set
    JNE,        // jumps to address if flag is not set


    ADD,        // adds two values
    SUB,        // subtracts two values
    MUL,        // multiplies two values
    DIV,        // divides two values
    INC,        // increments a value
    DEC,        // decrements a value

    AND,        // performs bitwise AND on two values
    NAND,       // performs bitwise NAND on two values
    OR ,        // performs bitwise OR on two values
    XOR,        // performs bitwise XOR on two values
    NOT,        // performs bitwise NOT on a value
    SHL,        // shifts a value left by a number of bits
    SHR,        // shifts a value right by a number of bits
}
