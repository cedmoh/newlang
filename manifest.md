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
x int var
```

Immutable Uninitialized Variable Declaration with Explicit Type:

```
x int val
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
x int val 42
```

Mutable Variable Initialization with Explicit Type:

```
x val int 100
```

Variable Initialization with Expression:

```
x val 2 + 2
```

```
x val (2 * 3) + (4 / 2)
```

Variable Initialization with Multiline Block Expression:

```
x val {
  2 + 2
}
```

Variable Initialization with Function Call:

```
x val myFunction
```

Variable Initialization with Identifier:

```
x val y
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
x boo val y
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
offsetX dec val 99.95
```

```
offsetY dec val -1_234.005
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
(2 + 2, sine(10), width)
```

Tuple Literal with Function Call as Member:

```
(myFunction(param1, param2), myFunction2(param1))
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

Tuple Literal with Named Fields and Function Call:

```
(width {getWidth square}, height {getHeight square})
```

Tuple Literal with Named Fields and Expression:

```
(class if getIsVisible() { 'visible' } else { 'hidden' },)
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
    x, y, z do todo,
}
```

Tuple Pattern Matching Target:

```
match x, y {
  _ do todo,
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
array val [1, 2, 3, 4, 5]
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

```
myMatrix
| 1, 2, 3 |
| 4, 5, 6 |
| 7, 8, 9 |
```

#### Access in Expression

```
myMatrix[0, 1]
```

# Blocks

Block with Empty Body:

```
{}
```

Block with Function Call:

```
{
  print 'Hello, World!'
}
```

Saving Block Result to Variable:

```
result val
  {
    'result'
  }
```

Breaking Out of a Block:

```
{
  if true { br }
  print 'This will never run'
}
```

Saving Block Result with Break Condition to Variable:

```
result val
  {
    if true { br 'Result' }
  }
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
myFunction()
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

# Control Flow

## If

Single if Statement:

```
if true {
  print 'Executed!'
}
```

if Statement with else Clause:

```
if false {
  print 'Not executed :('
} else {
  print 'Executed!'
}
```

if Statement with else-if Clause:

```
if false {
  print 'Not executed :('
}
elsif true {
  print 'Executed!'
}
```

Result of if Statement Saving to Variable:

```
result val
  if true { 'foo' }
  else { 'bar' }
```

## Match

Match Statement with Single Arm:

```
match myValue {
  1 do print 'One',
}
```

Match Statement with Multiple Arms:

```
match myValue {
  1 do print 'One',
  2 do print 'Two',
}
```

Match Statement with Default Arm:

```
match myValue {
  1 do print 'One',
  2 do print 'Two',
  _ do print 'Other',
}
```

Match Statement with Pattern Matching:

```
match x, y {
  0, 0 do print 'Both Zero',
  1, 1 do print 'Both One',
  _ do print 'Not the Same',
}
```

Saving Match Result to Variable:

```
result val
  match myValue {
    true do 'Yes',
    false do 'No',
  }
```

## Loop

Loop Statement:

```
loop {
  print 'This will run forever!'
}
```

Loop with Break Condition:

```
loop {
  if true { br }
  print 'This will never run'
}
```

Loop with Continue Condition:

```
loop {
  print 'Looping forever!'
  if true { cont }
  print 'This will never run'
}
```

Saving Loop Result to Variable:

```
result val
  loop {
    br 'Result'
  }
```

## While

While Loop with Condition:

```
x var 0
while x < 10 {
  x += 1
  print 'This will run forever until x reaches 10!'
}
```

While Loop with Break Condition:

```
while true {
  if true { br }
  print 'This will never run'
}
```

While Loop with Continue Condition:

```
while true {
  print 'Looping forever!'
  if true { cont }
  print 'This will never run'
}
```

Saving While Loop Result to Variable:

```
result val
  while true {
    br 'Result'
  }
```

## For-In

for-in Loop with Range:

```
for i in 0..10 {
  print i
}
```

for-in Loop with Array:

```
for item in [1, 2, 3, 4, 5] {
  print item
}
```

for-in Loop with Tuple:

```
for item in (1, 2, 3) {
  print item
}
```

for-in Loop with break Condition:

```
for item in array {
  if item == 2 { br }
  print item
}
```

Saving for-in Loop Result to Variable:

```
result val
  for item in array {
    if item == 2 { br item }
  }
```

## For

Incrementing for Loop:

```
for i var 0, i < 10, inc i {
  print i
}
```

Decrementing for Loop:

```
for i var 10, i > 0, dec i {
  print i
}
```

Decrementing for Loop with Break Condition:

```
for i var 10, i > 0, dec i {
  if i == 5 { br }
  print i
}
```

Saving for Loop Result to Variable:

```
result val
  for i var 0, i < 10, inc i {
    if i == 5 { br i }
  }
```

## Pipe

Pipe Operator for Function Call:

```
myFn pipe myOtherFn it
```

Pipe Operator for Function Call with Multiple Arguments:

```
10 pipe calculate it, it * 2
```
