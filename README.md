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
    asm     Assemble asm file to binary file for VM
    help    Prints this message or the help of the given subcommand(s)
    run     Run pc file on VM
```

## Error reporting
Full support for detailed errors is not yet done. This is how it looks now:
```
  × Syntax error encountered.
   ╭─[file.pca:5:1]
 4 │ 
 5 │ mv a, 3
   · ▲
   · ╰── expected "rts"
 6 │ 
   ╰────

Error:   × Syntax error encountered.
   ╭─[file.pca:5:1]
 4 │ 
 5 │ mv a, 3
   · ▲
   · ╰── while parsing Many1
 6 │ 
   ╰────
```

## Roadmap

- General
    - [ ] Redo error handling (again)
    - [x] Restructure project into workspace with multiple crates
    - [ ] Refactor code
- Parser
    - [x] Basic syntax
    - [x] Comments
    - [x] Labels
    - [x] Underscore character in numbers for better readability
    - [x] Plus character in numbers
    - [x] Negative numbers numbers
    - [ ] ORG instruction should be optional (default 8)
    - [x] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV
            - [ ] Type 3 (arrays)
            - [ ] Type 4 (arrays)
        - [x] Arithmetic
        - [x] Branch
        - [x] I/O
        - [x] Subroutine
- Assembler
    - [x] Assembles basic programs
    - [x] Value and Address references (two word support)
    - [ ] Raise syntax error instead of silently assembling some side effect
    - [x] Complete instructionset
        - [x] STOP
        - [x] ORG
        - [x] MOV
            - [ ] Type 3 (arrays)
            - [ ] Type 4 (arrays)
        - [x] Arithmetic
        - [x] Branch
            - [ ] null argument for arg1 or arg2 (WIP)
        - [x] I/O
        - [x] Subroutine
- VM
    - [ ] Run basic programs
    - [ ] Debugger
    - [ ] Complete instructionset
        - [x] STOP
        - [ ] MOV
            - [ ] Type 3 (arrays)
            - [ ] Type 4 (arrays)
        - [ ] Arithmetic
        - [ ] Branch
            - [ ] null argument for arg1 or arg2 (WIP)
        - [ ] I/O
            - [x] OUT
            - [ ] IN
        - [ ] Subroutine

## Sources

[MessyLab documentation](https://messylab.com/pico/)  
[ETF Presentation](https://rti.etf.bg.ac.rs/rti/ir1p1/materijali/predavanja/P1_2_pico_computer.pdf)

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).  
You may copy, distribute, and modify the software as long as you track changes/dates in source files. Any modifications to this project must also be open-sourced under the GPL-3.0 License.
