# Whitespace Documentation

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
 1  | 

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
```

### nv $1
where $l is the index

Stack:
```
  |i|
  | |
  | |
  | |
  |-|
```
