# Masm
Malik's assembly compiler.

## Sections
Supported sections are: 
- data: define identifiers
- text: executable instructions

## Data Definitions
Data definitions are built like this: ```name type value```.
- ```name``` is the identifier's name. 
- ```type``` is the type. Supported types are ```db``` (define bytes -> &[u8]) and ```dw``` (define word -> u16).
- ```value``` is the value associated with ```name```. It must match the promised ```type```. 

## Comments
Comments start with ```--```. Note that comments always occupy a whole line. 
There is currently no support for comments that start not as the first character in a line.

## Examples
```
section .data
msg db "Hello World"
len dw 11

section .text

-- This is the entrypoint.
_start:
-- Move the value 34 into RA
  MOV RA $34
-- Jump to the label.
  JMP my_label

my_label:
  MOV RA $50
  NOP

_stop:
  MOV RA RB
  HLT
```

