# Core Fundamentals

## Variables

- Declaration: `let mut x;`.
- Default behaviour: Immutable.
- Can be made mutable using the keyword `mut`.
  - Ex: let mut x = 5;

## Constants

- Declaration: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`.
- Keyword `mut` cannot be used.
- Behaviour: always immutable.
- Naming Convention: UPPERCASE.
- Valid & Scope: Entirety of program and in declared scope.

## Shadowing

- It is implemented using the keyword `let` and doesn't have relations with mutability.
- Prevents having variants of variables
  - Ex: let spaces = " ";
  - Prevents creation of variables like `spaces_num, spaces_str` etc.

## Datatypes

- Rust is statically typed language.
- Datatypes two parts: Scalar and Compound.
  - **Scaler**
    - It is single value.
    - Four primary scalar types: _integers_, _floating-point numbers_, _booleans_ and _characters_.
    - All numeric operations are possible: +, - , \*, /, %.
  - **Compound**
    - Group multiple values in one.
    - Two types: _tuples_, _arrays_.

### Integer Types

| Length                 | Signed          | Unsigned |
| :--------------------- | :-------------- | :------- |
| 8-bit                  | i8              | u8       |
| 16-bit                 | i16             | u16      |
| 32-bit                 | i32 _(Default)_ | u32      |
| 64-bit                 | i64             | u64      |
| Architecture dependent | isize           | usize    |

- Signed variant capacity: -(2<sup>n-1</sup>) to 2<sup>n</sup> - 1.
- Unsigned variant capacity: 2<sup>n</sup> - 1.
- Possible Issue: Integer Overflow

### Float Types

| Length | Type            |
| :----- | :-------------- |
| 32-bit | f32             |
| 64-bit | f64 _(Default)_ |

### Boolean Type

- Declaration: `let mut isLogin: bool = false;`.
- Type: bool
- Length: 8-bit (1 byte)

### Char Type

- Declaration: `let z: char = 'z';`.
- Type: char
- Length: 32-bit (4 bytes)
- Standard: Unicode
- Value declared in single quotes -> ''.

### Tuple Type

- Declaration: `let tup: (i32, f64, u8) = (500, 6.4, 1);`.
- Holds values of different/same datatypes.
- Fixed length: Cannot be extended.
- Use destructuring to retreive a single value.
- Use 0-based indexing to fetch an element in the tuple.

### Array Type

- Declaration: `let a = [1, 2, 3, 4, 5];`.
- Hold values of same datatype.
- Fixed length as compared to vector in Standard Library.
- Memory Allocation: Stack.
