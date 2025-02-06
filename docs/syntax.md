# Syntax

The syntax will be LISP like, in that it goes in the form `(function args)`.
Many functions will be of the form `(function target arg)`.
For example: `(+ A B)` will add B to A.

## NOTE

For now, functions will be given full names, but in the future, they will have single letter names for ease of parsing. 

## Arguments

Constants and other arguments are notated with a function, for now seen as the 


## Functions

### (N <N>)

sets N to the specified value. Creates the arrays.
Internally, there will be two more arrays than specified, because the first array is the regular variables, and the second is tmp.

### (w (condition) (code))

while the condition evaluates to true, execute the code

### (c comment)

ignore - anything inside this is a comment, and should not be evaluated

### (f (condition) (code))

if the condition evaluates to true, execute the code.

### (+ (target) (summand))

adds the summand to the target variable

### (- (target) (subtrahend))

subtracts the subtrahend from the target variable

### (> (left) (right))

returns true if `left > right`

### (< (left) (right))

returns true if `left > right`

### (= (left) (right))

returns true if `left = right`

### (| (left) (right))

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

### (@ (array) (index))

returns the variable at the given index of the given array (numerically assigned).
If the index is greater than the length of the array, the last item is given.
This can be used instead of specifying a variable name, ie, it's a source.

### (^ (name) (value))

creates a constant with the given value. Constants cannot be changed ever.
