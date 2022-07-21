# unnamed_cpu_emulator

## what is this project?

this is a virtual machine, similar to the jvm. the assembly language is based very heavily off x86 intel syntax assembly.


## why?

it was an idea i had while bored, and got me really interested. in a few days, i had a working set of instructions, registers, flags, and a working stack and ram. then a working way to execute instructions, and the day after a way to parse a file into a series of numbers which can be executed by the vm.


## how does it work?

very similar to assembly language. it takes a set of instructions, puts it in ram, and then points a register to where the instructions start. it executes the instructions as they come. using interrupts, you can write things to ram and print things to the console.

### the process

- programmer writes some code
- programmer assembles that code into a file of numbers with unnamed_cpu_assembler
- programmer sends those numbers to unnamed_cpu_runtime, which runs the program


## why in rust?

because rust is the language i am most comfortable in.


# todo before next minor release

- [ ] tests

- [ ] register address vs register with square brackets

- [ ] relative addressing

- [ ] escape code support
