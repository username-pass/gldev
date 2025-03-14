# The Big Function Definition Project

## Terminology

`o`   => refers to any value or thing
`$o`  => means that the thing is a parameter
`*k`  => refers to a constant. Useful for referring to variables by index.
`c`   => refers to a constant
`i`   => refers to an array by index
`j`   => refers to a variable by index
`k`   => refers to a cell by index
`N`   => refers to the number of arrays, constant set at compile time
`l`   => refers to a label, such as a variable name, or a macro name
`arr` => the main array of all variables and arrays
  Ex: `arr[i]`       => The ith array
      `arr[i][j]`    => The jth var of the ith array
      `arr[i][j][k]` => The kth cell of the jth var of the ith array
`WC1` => the first work cell, used more for calculations and returning
`WC2` => the seceond work cell, used for calculations
`li`  => the last referred to array
`lj`  => the last referred to variable
`WC`  => refers to a "work cell"
`cell`=> the discrete unit inside a variable


## The Architecture

The bytecode runs in a sort of virtual machine, with the following properties:

1) All variables are a part of a 2d array wher one dimension is the variable,
    and the other is the specified array. Think of it like you have various
    arrays of any size, and you can refer to individual items in the array.
    All regular variables are in the first array.
    Example:
    `newvar A` => corresponds to `arr[i][A]`
2) Each variable has its own set of `cells`, which actually contain the values.
    They can contain positive values up to 256. No support for negative numbers
    exists, numbers > 256 will loop to 0.
    Example uses:
      - if one variable were to correspond to a string, each cell could be a
        char.
      - if one variable were to refer to a base-10 integer, one cell could
        correspond to a digit
      - etc.
    For simple variables, such as `char`s, one may mostly just use cell #1 
2) To perform operations, there are two work cells, to store temp variables,
    and perform arithmetic.
    Example:
    To add A and B's nth cell, you first push their cell values to the two WCs.
    Then, add the WCs, and push the new value to whichever variable you wish to
    store the value in.
3) The machine has two main pointers, constantly set (defult: `arr[0][0]`)
    They are `i`, `j`, and `k`, which point to the current variable at any given
    time. (`arr[i][j][k]`) They can exist independently, so you can change
    variable index without changing array, and vice versa. Any operation you do
    with a variable, such as loading, putting, etc. are to do with the
    specified variable

## The bytecode syntax

Here are all the commands, with their definitions in what they do:

### macro $l $1 $2 ... $n

starts a macro definition.
Anything after this will be part of a macro, until the *matching* `endmacro`
(You can have nested macros, technically)
Every variable used will be replaced with the value given, inline.

Example:
```
  macro hello $name
    print $name
    # ...
  endmacro

  hello test
  # => turns to: 
  # print test
  # ...
```

### endmacro $l

ends the specified macro definition.
It must have started the definition earlier, otherwise things go bad!
It matches the one specified, so
```
  macro hi
    macro hello
    endmacro hi
  endmacro hello
```
technically means that `hi` turns to `macro hello; endmacro hi`, while `hello`
turns to `endmacro hi`
This is NOT good practice, please don't do it unless you have a *really* good
reason!

### init $N

inits the state and everything.
Sets up `N` arrays, partitions memory, etc.
This should be the first code line in the code.
`N` should be greater than 1, unless you don't want any variables for some
reason

### newvar $l

creates a new variable with name `$l` in the curently specified array.
From then on, `$l` will refer to that specific variable, or at least its index.
Technically you could do something like

```
  jta A
  # assume first item is referring to the first item in the array
  newvar first_item
  # ...
  jta B
  jtv first_item
  # It is now pointing to the first item of B
```

### jta $l/*j

sets `i` to the specified array. You can replace `$l` with `*i` as well
think of it like `i = *i` or `i = value($l)`

### jtv $l/*j

same thing as with `jta`, but with `j`, and variables, instead of arrays.

### jtc $l/*j

same thing as with `jta`, but with `k`, and cell indices, instead of arrays.

### while

If `WC1` is non-zero, starts a loop.

### end

if `WC1` is zero, exits, does nothing.
if `WC1` is non-zero, continues program execution at matching `while`,
continuing the loop

### load

loads the `k`th cell to `WC1`, replacing value.
`WC1 = arr[i][j][k]`

### swap

swaps `WC1` and `WC2` values
`WC1, WC2 = WC2, WC1`

### sub

subtracts `WC2` from `WC1`, setting `WC1` to the new value. It stops at 0,
`WC1 = max(WC1 - WC2, 0)`

### put

sets the current cell to `WC1`, overriding the value
`arr[i][j][k] = WC1`

### add_c $c

adds a constant `$c` to `WC1`
`WC1 = WC1 + c`

### sub_c $c

subtracts a constant `$c` from `WC1`
same rules as other subtraction. Ends at 0, no negatives.
`WC1 = max(WC1 - c, 0)`

### set $c

sets `WC1` to a constant `$c`
`WC1 = c`

### clear

sets `WC1` to 0, the same as `set 0`

### printc

prints `WC1` to stdout as ascii value. (So, to print `A`, `WC1` should be `65`)

### input

inputs the next char/byte from stdin, sets `WC1` to the new value. A bit like
load, except from stdin instead of a variable

### error

prints out `error`. Useful for stating when behaviour is not supposed to happen

## The Abstract

In my language, functions work as such:
`(functionname arg1 arg2 ...)`
where `functionname` is the name of the function, and each arg can be their own
functions as well, or just standalone values.
Functions are defined in the language through an intermediary string language
thing. It's weird, and you don't have to worry about it too much. If you really
want, here is an example:

Example:
### (while (cond) (code))
```
while ->       "whilenxnppeswhile\n npeesend\n q"
"w"           -> this is for the `w` command
"n"           -> move to next param (default)
"x"           -> do nothing by default to any extra params
"n"           -> move to next param (cond)
"p"           -> push val to stack
"pe"          -> evaluate condition
"s\nwhile\n " -> writes "while\n"
"n"           -> goes to next parameter
"pe"          -> evaluates code
"s\n "        -> adds space between code and condition
"e"           -> pops and evaluates condition
"s\nend\n "   -> writes "end\n"
"q"           -> finishes definition
```


The basic idea of that the string thing turns the function into usable bytecode
I will deal with that.

The bigger issue is finding out *what* the usable bytecode is.
For example, a `while` function would turn to
```
  (condition)
  while
    (code)
    (condition)
  end
```

Things can get complicated, quick.

Macros are an important part of the bytecode, because they allow useful
abstractions. For example, the bytecode for `(+ A B)` would just be
```
  (A)
  (B)
  add
```
where it evaluates A and B, presumably jumping to them as well.
This allows the addition function to work with any type. The key is, that
whenever you call a value, and it returns something, it must call the macro
definitions for that type, redefining them to do what they should for that
type. For example, if you do `(char A)`, loading it as an 8-bit char, the
definition for `add` might look something like this:
```
  load          # load the current variable
  jump_to_last  # load the last variable through a macro
  swap          # swap, freeing up WC1 for the new value
  load          # load the new value in
  add           # add the two
  put           # put the result back into A
```

for an int (where each cell is a digit), however, it would look more
complicated, as one needs to account for the carrying over of the digits. For
more complicated data structures, it gets more complicated for other functions
as well.

That function, `char` sets up all the macros for the given data type, in this
case char. There are a couple macros needed for all the data types, so that
things function smoothly


## The Mission

You mission, should you choose to accept it, is to help me create the bytecode
function definitions for my language, thereby helping me complete my CS final.

The example would be the while loop, and the bytecode definition.
I have a list of things I need definitions for. If you could help me make some,
that would be great. 

## The List

Here is the list of things I need:

### Data types

The basic macros for each data type. Here are the data types:
1) char
 - the simplest one, it just stores a single char in the 1st cell of the given
  variable. Addition and subtraction are just regular addition and subtraction
  for the value. Same goes for the other functions
 - when printed, prints as the specific ascii char
2) base-10 int
 - An int where each slot represents a digit in base-10
 - when printed, print each digit as the digit.
  Examples: cells: [1,4,9], output: 149
 - when multiplied or divided, perform integer division with each digit as
  needed. (Divide but discard remainder, to get remainder, use modulo)
3) string
 - a string of a fixed length, where each cell is a char
 - no addition or subtraction
 - can set specific values
4) float
 - A floating point number, where one cell is the sign, one is the exponent,
  and one (or more) cells for the mantissa
5) boolean
 - a value that is either true or false
6) pointer
 - stores the i,j, and k of another variable in its cells
 - calling returns the given variable
 - addition and subtraction add to the pointer reference, not the actual
  value of the variable



### Needed functions

For every data type, the following functions are needed:

#### calling function

The calling function calls a variable by index, and sets the currently selected
variable to the specified one. It also sets all the macros needed for any
future functions, such as `add`, `subtract`, `print`, etc.

#### conditionals

A conditional would go in the following way:
`(if (condition) (true-case) (false case [optional]) )`



#### variable creation

Variable creation would have their own creation function for each data type, or
follow whatever way you implement the data types. They create a new variable
with the specified name, which can then be accessed when a calling function is
called with the same name. It formats the variable accordingly as well


### Example program:

#### Fibonacci

init 0
newvar tmp
newvar counter
jtv tmp
clear
add 1
put 1
swap
set 10
while
  put 1
  jtv tmp
  load 1
  add
  swap
  put 1
  jtv counter
  load 1
  sub_c 1
end
