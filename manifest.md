# Comments

Single-Line Comment:

```
# this is a single line comment
```

Multi-Line Comment:

```
###
This is a multi line comment
Also used for documentation
###
```

Inline Comment:

```
## this is an inline comment ##
```

# Variables

Immutable Variable Declaration:

```
x
```

**NOTE** Immutable variable declaration without initialization is illegal and will not compile.

Mutable Variable Declaration:

```
mut x
```

Immutable Variable Declaration with Explicit Type:

```
x i32
```

Immutable Variable Initialization:

```
x "hello"
```

Mutable Variable Initialization:

```
mut x "hello"
```

Immutable Variable Initialization with Explicit Type:

```
x i32 42
```

Mutable Variable Initialization with Explicit Type:

```
mut x u32 100
```

Variable Initialization with Expression:

```
x 2 + 2
```

Variable Initialization with Multiline Block Expression:

```
x {
  2 + 2
}
```

# Operators

## Unary Operators

### Arithmetic

Negation: `-`

### Logical

Logical Not: `!`

## Binary Operators

Assignment: `=`

### Arithmetic

Addition: `+`

Subtraction: `-`

Multiplication: `*`

Division: `/`

Modulus: `%`

Exponent: `**`

### Arithmetic Assignment

Addition Assignment: `+=`

Subtraction Assignment: `-=`

Multiplication Assignment : `*=`

Division Assignment: `/=`

Modulus Assignment: `%=`

### Comparison

Greater Than: `>`

Less Than: `<`

Greater Than or Equal: `>=`

Less Than or Equal: `<=`

Equal to: `==`

Not Equal to: `!=`

### Logical

Logical AND: `&&`

Logical OR: `||`

### Logical Assignment

Logical AND Assignment: `&&=`

Logical OR Assignment: `||=`

### Bitwise

Bitwise AND: `&`

Bitwise OR: `|`

Bitwise XOR: `^`

Left Shift: `<<`

Right Shift: `>>`

Unsigned Right Shift: `>>>`

Bitwise Not: `~`

### Bitwise Assignment

Bitwise AND Assignment: `&=`

Bitwise OR Assignment: `|=`

Bitwise XOR Assignment: `^=`

### Shift Assignment

Left Shift Assignment: `<<=`

Right Shift Assignment `>>=`

Unsigned Right Shift Assignment `>>>=`

# Data Types

## Scalar

### Integer

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Integer Literals:

| Literal | Example        |
| ------- | -------------- |
| Decimal | 98_222         |
| Hex     | 0xff           |
| Octal   | 0o77           |
| Binary  | 0b1111_0000    |
| Byte    | (u8 only) b'A' |

Integer Value in Assignment:

```
x 42
```

```
color u32 0xff0055
```

```
mask i32 0b1101_0010
```

### Float

| Length | Type |
| ------ | ---- |
| 32-bit | f32  |
| 64-bit | f64  |

Float Value in Assignment:

```
offsetX f32 100.
```

```
offsetY f64 42.005
```

### Boolean

True value: `true`

False Value: `false`

Boolean Value in Assignment:

```
bool x false
```

## Compound

### Tuple

Tuple Type and Literal Value in Assignment:

```
(i32, f64, u8) tuple (500, 6.4, 2)
```

Tuple Value Access:

```
tuple.0
```

### Array

Array Type and Literal Value in Assignment:

```
i32[5] array [1, 2, 3, 4, 5]
```

# Functions

## Declaration

Function Declaration:

```
f () {}
```

Function with Parameters

```
f (a i32, b i32) {}
```

Function with
