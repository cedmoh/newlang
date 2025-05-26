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

Immutable Uninitialized Variable Declaration:

```
x val
```

**NOTE:** Immutable variable declaration without initialization is illegal and will not compile.

Mutable Uninitialized Variable Declaration:

```
x var i32
```

Immutable Uninitialized Variable Declaration with Explicit Type:

```
x val i32
```

**NOTE:** Immutable variable declaration without initialization is illegal and will not compile.

Immutable Variable Initialization with Literal:

```
x val 'hello'
```

Mutable Variable Declaration Initialized with Literal:

```
x var 'hello'
```

Immutable Variable Declaration Initialized with Literal Specifying Explicit Type:

```
x val int 42
```

Mutable Variable Initialization with Explicit Type:

```
x val int 100
```

Variable Initialization with Expression:

```
x val = 2 + 2
```

Variable Initialization with Multiline Block Expression:

```
x val {
  2 + 2
}
```

Variable Initialization with Function Call:

```
x val = myFunction
```

Variable Initialization with Identifier:

```
x val = y
```

# Operators

## Unary Operators

### Arithmetic

Negation: `-`

### Logical

Logical Not: `not` or `!`

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

Equal to: `eq` or `==`

Not Equal to: `neq` or `!=`

### Logical

Logical AND: `and` or `&&`

Logical OR: `or` or `||`

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

### Boolean

True value: `true`

False Value: `false`

Boolean Value in Literal Assignment:

```
x val false
```

Boolean Value in Assignment:

```
x val bool = y
```

### Numeric

#### Integer

Integer in Literal Assignment:

```
x val 42
```

```
y val -100_234
```

#### Decimal

Decimal in Literal Assignment:

```
offsetX val f32 99.95
```

```
offsetY f64 -1_234.005
```

#### Hexadecimal

Hexadecimal in Literal Assignment:

```
color val #FF0099
```

```
alpha val #000000FF
```

#### Binary

Binary in Literal Assignment:

```
num val b0010
```

```
mask val b_1010100
```

#### Octal

Octal in Literal Assignment:

```
octalValue val o0755
```

```
octalValue val o_0755
```

### Character

Character in Literal Assignment:

```
char val c'A'
```

**NOTE:** Currently implemented as `^A`.

## Compound

### String

String in Literal Assignment:

```
msg val 'Hello, World!'
```

String with Escape Sequences:

```
msg val 'Hello, \nWorld!'
```

String with Unicode Characters:

```
msg val 'Hello, 世界!'
```

### Tuple

#### Type Definition

Tuple Type Definition:

```
(i32 f64 u8)
```

```
(String)
```

Tuple Type Definition with Named Fields:

```
(width u32, height u32, zIndex i32)
```

```
(size u32,)
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

Tuple Literal with Literals as Members:

```
(1, 2, 3)
```

Tuple Literal with Literals of Different Types:

```
(1, 2.5, 'c')
```

Tuple Literal with Evaluated Members:

```
(2 + 2, sine 10, width)
```

Tuple Literal with Nested Tuples:

```
(1, (2, 3), 4)
```

Tuple Literal with Named Fields:

```
(zIndex 1, width 100, height 200)
```

Tuple Literal with Named Fields and Mixed Types:

```
(width 100, height 200, zIndex 'z')
```

Tuple Literal with Named Fields and Nested Tuples:

```
(size, (width 100, height 200), zIndex 1)
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

Tuple Pattern Matching Arm:

```
match myTuple {
    x, y, z do todo
}
```

Tuple Pattern Matching Target:

```
match x, y {
  _ do todo
}
```

Tuple Destructuring with Named Fields:

```
x, y, z = myFunction
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

#### Pattern Matching

Array Pattern Matching Arm:

```
match myArray {
    x, y, z do todo
}
```

```
match myArray {
    x, .. do todo
}
```

```
match myArray {
    x, ..y do todo
}
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

# Functions

## Declaration

Function Declaration with Empty Body:

```
myFunction fn
```

```
myFunction fn {}
```

Function Declaration with Single Line Body:

```
myFunction fn do print 'Hello!'
```

Function Declaration with Multiple Lines Defined Using `do`:

```
myFunction fn do print 'Hello' do print 'World!'
```

Function with a return value

```
myFunction fn {
  ret 0
}
```

Function with Multiple Arguments:

```
myFunction fn arg1, arg2 {}
```

Function with Function Type and Multiple Arguments:

```
myFunction FunctionType fn arg1, arg2 {}
```

Function with Typed Arguments:

```
myFunction fn arg1 ArgOneType, arg2 ArgTwoType {}
```

Function with Return Type:

```
myFunction fn -> ReturnType {}
```

Function with Multiple Arguments and Return Type:

```
myFunction fn arg1, arg2 -> ReturnType {}
```

Function with Generic Type in Argument and Return Type:

```
myFunction fn<T> arg2 GenericArgument<T> -> GenericReturnType<T> {}
```

Function with Multiple Generic Types:

```
myFunction fn<T, U> {}
```

Function with Multiple Generic Types and Constraints:

```
myFunction fn<T TypeConstraint + AnotherTypeConstraint, U> arg {}
```

## Function Call

Function Call with No Arguments:

```
myFunction
```

Function Call with one Parameter:

```
myFunction param
```

Function Call with Multiple Arguments:

```
myFunction param1, param2
```

Function Call with Explicit Generic Type:

```
myFunction<GenericParam> param
```

Function Call with Multiple Explicit Generic Types:

```
myFunction<GenericParam1 GenericParam2> param1 param2
```

# Decorators

## Function Decorators

Function Decorator with No Parameters:

```
@myDecorator
myFunction fn
```

Function Decorator with Parameters:

```
@myDecorator decoratorParam1, decoratorParam2
myFunction fn
```

Multiple Function Decorators on the Same Function:

```
@firstDecorator decoratorParam
@secondDecorator
myFunction fn
```

# Closures

## Declaration

Closure Declaration with Empty Body:

```
myLambda () {}
```

Closure Declaration with Arguments:

```
myLambda (arg1, arg2) {}
```

# Structs

## Declaration

Empty Struct Declaration:

```
myStruct struct
```

```
myStruct struct {}
```

Struct Declaration with Fields:

```
myStruct struct {
  field Type
  anotherField AnotherType
}
```

Generic Struct Declaration:

```
myGenericStruct struct<T> {
  field GenericType<T>
}
```

# Enums

## Declaration

Enum Declaration:

```
myEnum enum {
  Variant
  AnotherVariant
}
```

Enum Declaration with Custom Variant Values:

```
myEnum enum {
  Variant 1
  AnotherVariant 2
}
```

```
myEnum enum {
  Variant 'value'
  AnotherVariant 'another value'
}
```

Enum Declaration with Loaded Variants:

```
myEnum enum {
  Variant(width u32, height u32)
  AnotherVariant(string number)
}
```
