# Whitespace Documentation

current link:
`https://excalidraw.com/#json=WkTOF3UD4UKdp0d5mXr44,B1MH5YNiUbY3TONYDXEX6Q`

Compilation to whitespace will be through the following.

## Variable storage

Variables will be stored through the heap.


### Intro to whitespace heap

The Heap is just a key-value storage in whitespace, that allows storage of an
arbitrary i32 at the address. The key is also an i32, specifying the location.

### Variable layout in memory

Each variable can take up 1 element in the heap.
Each 8 bit cell is composed inside the variable's index, using the formula
cell(n, idxval) {
  if n == 0 {
    return idxval;
  } else {
    return cell(n-1, (idxval - (idxval % 256)) / 256);
  }
}

Variables are stored with the following formula:

`arr[i][v] = v*N + i`

whitespace tests
```
push 1
push 1795 # 7 * 256 + 3
drop
push 459648 # ( 7 * 256 + 3 ) * 256 + 128

call start
end

# to find the nth cell of specified variable
# assume top of stack is n, and below it is the variable

abel start
swap
mul -1
label start_loop
swap
# go to next layer
	div 256

# decrement counter
# swap order
swap
add 1
dup
jn start_loop
ret

```

Arr order:

0         | 1         | ...       | n
0 1 ... N | 0 1 ... N | 0 1 ... N | 0 1 ... N

item 0 of array 0 is N
array 0 is all the constants and special variables
arrays 1, N are just arrays as defined by user

### Array 0 order:

idx | what it has
 0  | number of vars (len of arr 0)
 1  | work cell
... | work cells
 9  | work cell
### Array i item 0

item 0 of array i is just the length of array i


## Whitespace asm representations of bytecode

### init N

No stack prerequisites
```
  # N is stored at idx -1
  push -1
  push N
  # offset by one for the 0 array
  add 1
  store
  # make the currently selected array be 0
  push 0 
```

### nv $l
where $l is the index

Stack:
```
  |i| # i is arr number
```
does nothing for running purposes, only for compilation purposes and matching a value for the label $l to an index.

### jta $k
where $k is the array number
stack prerequisite:
```
  |i|
```
where `i` is the old arary number


outputs $k to the stack so that it can be used
```
  drop
  push $k
  
```

### jtv $k
Where $k is the index of the variable in the arr
Stack prereq:
```
  |i|
```
where i is the array to push to
```
  dup
  push $k
  call get_N
  mul
  add
```
returns the index of the variable desired on top of stack

output:
```
  |j|
  |i|
```
where `j` is the index of the variable specified

### load $l $k $k2

sets the $k2th WC to the $kth data cell at variable
$l of current array

```
# index of variable
push $l
# get variable value
call read_var_value
# now the stack is [i, val], where val is the variable val
push $k2
#extract the $k2th cell of var
call extract_var_cell
# stack: [i, cell]


```
