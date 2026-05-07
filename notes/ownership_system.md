# Ownership

## Owneship

- **Definition:** It is a set of rules that govern how a Rust program manages memory.

### Rules of Ownership

1. Each value in Rust has an _owner_.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

### Variable Scope

- **Definition:** The scope of a variable is the region of code where the variable is accessible.

### String Type

- It is a growable, UTF-8 encoded string stored on the heap.
- The operator `::` is used to call static methods on a type.

### Memory and Allocation

- The memory is autmatically returned when the variable goes out of scope.
- Rust calls a function called `drop` when the variable goes out of scope.
- _Double Free Memory Error:_ When a value is dropped more than once.
- _Move replaced Shallow Copy:_ When a value is moved from one variable to another, the old variable is no longer valid.

#### Scope and Assignment

- Method `clone()`: Clones a value, creating a new value that is a deep copy of the original.
  - Ex: `let s2 = s1.clone();` This creates a new string `s2` that is a deep copy of `s1`.

### Ownership and Functions

- Functions can take ownership of values, move them, and return ownership.
- For the values with known size, the compiler can copy them by value and for the values with unknown size, the compiler must move them.

### Returning Values and Scope

- For values with unkown size, the function return the ownership of the value to the caller.

## References and Borrowing

- It is a pointer that consists of the address of a value stored on the heap.
- It does not own the value it points to, it only borrows it.

### Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

### Mutable References

It allows you to modify the value it points to. Default reference is immutable.

- Ex: `let mut x = 5; let y = &mut x; *y = 10;` This creates a mutable reference `y` to `x` and modifies the value it points to.
- _Restriction of mutable reference:_ Multiple mutable references to the same value are not allowed. In other words, a value can only have one mutable reference at a time.
  - Tackles problem of data races and inconsistent memory.
  - Conditions for data races:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data
    - There’s no mechanism being used to synchronize access to the data.

### Dangling References

It is a reference referring to a memory location that has been freed or released.

- It can cause undefined behavior if used after the memory has been freed.

## Slices

- It lets to reference a contiguous sequence of elements in a collection.
- It is like a reference and doesn't have ownership.
- Range syntax:
  - `0..n`: It includes all characters from the start to the end of the string.
  - `..n`: It includes all characters from the start to the `n`th character.
  - `n..`: It includes all characters from the `n`th character to the end of the string.
  - `..`: It includes all characters from the start to the end of the string.

### Slice Types

1. **String Slices**
   - It is a reference to a contiguous sequence of characters in a `String`.
2. **Array Slices**
   - It is a reference to a contiguous sequence of elements in an array.
