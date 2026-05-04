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

## Functions

- `main` function is the entry point.
- Declaration: keyword `fn`.
  - Ex: `fn another_function(){};`.
- Naming convetion: "snake_case".
- Functions can be declared anywhere within the scope of the caller function.
- Rust is an "Expression-Based" language.
- **Statements and Expressions**
  - _Statements:_ These are instructions that perform some action and do not return a value.
    - Function definition is also a statement.
    - Cannot assign a `let` statement to another variable, as the following code tries to do; get an error:

    ```Rust
    fn main() {
        let x = (let y = 6);
    }
    ```

    - So in Rust `let` doesn't return a value. So assignment of multiple variables with equal value can't be done like "x = y = 8" unlike other languages.

  - _Expressions:_ It evaluate to a resultant value.
    - Expressions doesn't end with a semi-colon if done then it will be a statement and there might be error.

- **Functions with return values**
  - It is mandatory to mention the return type in the function declaration with the help of `->`.
    - Ex: `fn five() -> i32 { 5 };`.

## Comments

- Comments are done using `//` for single as well as multiple.

## Control Flow

- **if Expressions**
- Declaration: Keyword `if`.
- Block of code associated with the conditions in if expressions are sometimes also called "arms" similar to arms in "match".

## Loops

- 3 types: loop, while and for.
- Syntax
  - Loop: `loop { ... }`.
  - while: `while <condition> { ... }`.
  - for: `for i in (1..5) { ... }`.
- Keyword `break` to break the loop.
- Keyword `continue` to skip to the next iteration.
- Loop label should be begin with a single quote.
