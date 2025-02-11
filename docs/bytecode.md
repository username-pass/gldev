# Syntax

The syntax will be LISP like, in that it goes in the form `(function args)`.
Many functions will be of the form `(function target arg)`.
For example: `(+ A B)` will add B to A.

## NOTE

Function definitions use the following to define them
`(` can perform:
push: `push (str) of token to bytecode stack`
nop: `nothing`

`)` can perform:
pop: `pops the top of the stack, and evaluates it`
write: `writes a string directly to bytecode.`
neval: `writes the token to bytecode without evaluating`
nop: `nothing`

Bytecode will be created in the following way:
push: `stuff to push to bytecode stack`
write: `stuff to write to bytecode stack`

push adds to stack, write writes from stack. If you don't push anything, push a null thing, so that the stack is still correct.

This file is still a massive WIP

Example:
### (w (cond) (code))
w ->
w
(
push
)
eval
write "while\n"
(
)
eval
pop
write "end\n"

## NOTE

For now, I am going to treat variables as simple, and copy all of their respective bytes

## Arguments

Constants and other arguments are notated with a function, for now seen as the 


## Functions

### (N <N>)

N
write "init "
neval

### (w (condition) (code))

(condition)
while
  (code)
  (condition)
end

### (c comment)

<nothing>

### (f (condition) (code))

(
)
eval
write "while\n"
(
)
eval
write "lz\n"
write "end\n"


(condition)
while
  (code)
  lz
end      ; ensures that it is false

### (+ (target) (summand))

(
)
eval
(
)
eval
write "add\n"

(target)
(summand)
add       ; (macro defined by summand)

### (- (target) (subtrahend))


(
)
eval
(
)
eval
write "sub\n"

(target)
(subtrahend)
sub       ; (macro defined by subtrahend)

subtracts the subtrahend from the target variable

### (> (left) (right))

(
)
eval
(
)
eval
write "subwz\n"
write "cmp"

(left)
subwz (right) 1 1 ; (subtracts until 0, puts result in WC)

returns true if `left > right`

### (< (left) (right))


(
)
eval
(
)
eval
write "subwz\n"
write "cmp"

(right)
subwz (left) 1 1 ; (subtracts until 0, puts result in WC)
mvc 1        ; (move: AN -> WC)

returns true if `left > right`

### (= (left) (right))


(
)
eval
(
)
eval
write "sub\n"
write "cmp"

(left)
sub (right) 1 1 ; subtracts
mvc 1

returns true if `left = right`

### (| (left) (right))

(
)
eval
(
)
eval
write "add\n"
write "cmp"

returns true if either left is true or right is true

### (& (left) (right))

returns true if both left and right are true

### (! (condition))

returns true if condition is false

### (i (target))

takes the next byte of stdin and writes it to the target

### (o (source) (byte))

takes the specified byte of the source variable, and writes it to stdout

### (# (value))

returns a temporary variable with the value specified

### (: (target) (source))

sets the target to the value of the source

### (~ (name) (value/source))

creates a new variable with the specified value

### ($ (name))

returns the variable with the given name

### (@ (array) (index))

returns the variable at the given index of the given array (numerically assigned).
If the index is greater than the length of the array, the last item is given.
This can be used instead of specifying a variable name, ie, it's a source.

### (^ (name) (value))

creates a constant with the given value. Constants cannot be changed ever.
