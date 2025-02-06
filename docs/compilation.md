# Compilation

The compiler will work as follows:

1) consume next character
2) if next character is whitespace (0x09 or 0x20), then ignore, continue on
3) else, if character is open paren (0x28), add 1 to depth, set read type to command
4) else, if character is close paren (0x29), push bytecode from stack, set read type to command
5) else, if read type is command, read byte, push bytecode and write bytecode accordingly, set read type to param
6) else, if read type is param, add to bytecode accordingly, set read type to command

