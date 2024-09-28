# ricoComputer

Implementation of assembler & virtual machine for Dr Jozo Dujmović's picoComputer. 

## Usage

```
ricocomputer 0.1.0
Drop-in replacement for picoComputer assembler and virtual machine.

USAGE:
    ricocomputer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    compile    Compile asm file to binary file for VM
    help       Prints this message or the help of the given subcommand(s)
    run        Run pc file on VM
```

## Error reporting
Full support for detailed errors is not yet done. This is how it looks now:
```
Error:   × Syntax error encountered.
   ╭─[file:8:1]
 7 │ 
 8 │ this_instruction_doesnt_exist a
   · ▲
 9 │ stop (a)
   ╰────
```

## Roadmap

- Parser
    - [x] Basic syntax
    - [ ] Comments
    - [ ] Labels
    - [x] Underscore character in numbers for better readability
    - [x] Plus character in numbers
    - [x] Negative numbers numbers
    - [ ] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV
            - [ ] Type 3 (arrays)
            - [ ] Type 4 (arrays)
        - [x] Arithmetic
        - [x] Branch
        - [X] I/O
        - [ ] Subroutine
- Assembler
    - [x] Assembles basic programs
    - [x] Value and Address references (two word support)
    - [ ] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV
            - [ ] Type 3 (arrays)
            - [ ] Type 4 (arrays)
        - [x] Arithmetic
        - [x] Branch
            - [ ] null argument for arg1 or arg2 (WIP)
        - [X] I/O
        - [ ] Subroutine
- VM

## Sources

[MessyLab documentation](https://messylab.com/pico/)  
[ETF Presentation](https://rti.etf.bg.ac.rs/rti/ir1p1/materijali/predavanja/P1_2_pico_computer.pdf)

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).  
You may copy, distribute, and modify the software as long as you track changes/dates in source files. Any modifications to this project must also be open-sourced under the GPL-3.0 License.
