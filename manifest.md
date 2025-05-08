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
x 'hello'
```

Mutable Variable Initialization:

```
mut x 'hello'
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

Boolean Value in Literal Assignment:

```
x false
```

Boolean Value in Assignment:

```
x false = y
```

## Compound

### Tuple

#### Type Definition

Tuple Type Definition:

```
(i32 f64 u8)
```

Tuple Type Definition with Named Fields:

```
(width u32, height u32, zIndex i32)
```

Nested Tuple Type Definition:

```
((i32 f64) (u8 u16))
```

Tuple Type Definition with Generic Type:

```
myGenericFunction fn<T> -> (T T)
```

Tuple Type Definition with Generic Type and Named Fields:

```
myGenericFunction fn<T> -> (width T, height T)
``` 

#### Variable Definition

Tuple Value in Literal Assignment:

```
myTuple (1, 2, 3)
```

Tuple Variable Definition with Named Fields:

```
myTuple (zIndex 1, width 100, height 200)
``` 

Tuple Variable Definition with Type Cast:

```
myTuple (1, 2, 3) as MyTupleType
```



#### Access in Expression

Tuple Value Access with unnamed fields:

```
myTuple.0
```

Tuple Value Access with named fields:

```
myTuple.width
```

#### Pattern Matching

Tuple Pattern Matching:

```
match myTuple {
    (x, y, z) -> todo
}
```

### Array

#### Type Definition

Array Type Definition:

```
str[]
```

```
(u8, u8)[]
```

Nested Array Type Definition:

```
i32[][]
```

Array Type Definition with Length:

```
u8[16]
```

Array Value in Literal Assignment:

```
array [1, 2, 3, 4, 5]
```

### 2D Matrix

#### Type Definition

Matrix Type Definition:

```
i32[16 16]
```

#### Value in Literal Assignment:

myMatrix | 1, 2, 3 |
         | 4, 5, 6 |
         | 7, 8, 9 |

#### Access in Expression

```
myMatrix[0, 1]
```

### String

String literal

```
'This is a string literal'
```

# Functions

## Declaration

Function Declaration with Empty Body:

```
myFunction fn
```

```
myFunction fn {}
```

Function with a return value

```
myFunction fn {
    ret 0
}
```

myFunction fn param1, param2 {

}

myFunction FunctionType fn param1, param2 {

}

myFunction fn param1 ParamType, param2 Param2Type {

}

myFunction fn -> ReturnType {}

myFunction fn param1, param2 -> ReturnType {

}

function myFunction<T>(param1, param2: GenericParam<T>): GenericReturnType<T> {

}

myFunction fn<T> param1, param2 GenericParam<T> -> GenericReturnType<T> {
    
}

myFunction fn<T TypeConstraint + AnotherTypeConstraint, U> param {

}

myFunction  fn<T> param where TypeConstraint + AnotherTypeConstraint, U {

}

@myDecorator
myFunction fn

@myDecorator decoratorParam1, decoratorParam2
myFunction fn

@firstDecorator decoratorParam
@secondDecorator 
myFunction fn

myFunction

myFunction()

myFunction param1, param2

myFunction(param1, param2)

myFunction.call

myFunction<T>

myFunction<Type1 Type2 Type3> param1, param2, param3

myLambda () {}

# Function Call

myFunction

myFunction param

myFunction param1, param2

myFunction<GenericParam> param

myFunction<GenericParam1 GenericParam2> param1 param2



# Structs

myStruct struct

myStruct struct {}

myStruct struct {
    field Type
    anotherField AnotherType
}

# Enums 

myEnum enum {
    Variant
    AnotherVariant
}
