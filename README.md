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

## Roadmap

- Parser
    - [x] Basic syntax
    - [ ] Comments
    - [ ] Labels
    - [ ] Underscore character in numbers for better readability
    - [ ] Plus character in numbers
    - [ ] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV (Array MOV missing)
        - [ ] Arithmetic
        - [ ] Branch
        - [ ] I/O
        - [ ] Subroutine
- Assembler
    - [x] Assembles basic programs
    - [x] Value and Address references (two word support)
    - [ ] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV (Array MOV missing)
        - [ ] Arithmetic
        - [ ] Branch
        - [ ] I/O
        - [ ] Subroutine
- VM

## Sources

[MessyLab documentation](https://messylab.com/pico/)  
[ETF Presentation](https://rti.etf.bg.ac.rs/rti/ir1p1/materijali/predavanja/P1_2_pico_computer.pdf)

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).  
You may copy, distribute, and modify the software as long as you track changes/dates in source files. Any modifications to this project must also be open-sourced under the GPL-3.0 License.
