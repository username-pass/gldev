# Intermediate representation

This will compile down to bfasm and wsasm 

## Commands:

### init N

sets up the 0th variables, and creates N = N, creating N arrays for future use. 

### nz $l

creates a new variable with the label $l in the current array

### jta $k

sets the array specified by $k to be the current array to do things with 

### jtv $l

sets the variable specified by $l to be the current variable specified for changes

### set $k $k1 ... $kn

sets the current variable's value to be set to the specified bytes. $k is the 0th byte, $k1 is the 1st byte, etc.

### cp $l $k

copies the $kth byte of the varible $l to the $kth byte of the current variable

### mv $l $k

moves the $kth byte of $l to the $kth byte of the current variable

### add $l $k $k2

adds the $kth byte of $l to the $k2th byte of the current variable

### sub $l $k $k2

subtracts the $kth byte of $l to the $k2th byte of the current variable

### while

will loop until the variable pointed at it's 0th byte is non-zero

### end

ends a loop section



