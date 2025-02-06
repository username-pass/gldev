# Intermediate representation

This will compile down to bfasm and wsasm 

## Commands:

### macro $l

starts a macro definition with the label $l

### endmacro

ends the macro definition. From now on, every time $l is found, it will be replaced with the code in between the `macro` and `endmacro`

### init N

sets up the 0th variables, and creates N = N, creating N arrays for future use. 

### nv $l

creates a new variable with the label $l in the current array

### jta $k

sets the array specified by $k to be the current array to do things with 

### jtv $l

sets the variable specified by $l to be the current variable specified for changes

### load $l $k

sets WC to the value of the $kth DC at the variable $l

### wp $k

adds the WC to the $kth DC

### ws $k

subtracts the WC from the $kth DC (ends at 0)

### wf $k

sets the $kth DC to the value of WC, clearing the old value

### lz

sets the value of the WC to 0

### lval $k

sets the WC to $k

### while

will loop until the WC of the variable pointed at is zero

### end

ends a loop section



