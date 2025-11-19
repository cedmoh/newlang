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

Mutable Uninitialized Variable Declaration with Explicit Type:

```
x number var
```

**NOTE:** Uninitialized variable declaration requires the type to include `nil`.

Immutable Uninitialized Variable Declaration with Explicit Type:

```
x number val
```

**NOTE:** Immutable variable declaration without initialization is illegal and will not compile.

**NOTE:** Uninitialized variable declaration requires the type to include `nil`.

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
x number val 42
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

Exponent: `^`

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

### Map

#### Literal

Empty Map in Literal Assignment:

```
empty val []
```

Map with unnamed fields in Literal Assignment:

```
array val [ 'value1', 42, true ]
```

Map with named fields in Literal Assignment:

```
object val [
  name: 'Alice',
  age: 30,
  isMember: true,
]
```

Map with mixed named and unnamed fields in Literal Assignment:

```
mixed val [
  name: 'Alice',
  1,
  2,
  3,
  isMember: true,
]
```

Map with nested Map in Literal Assignment:

```
nested val [
  person: [
    name: 'Alice',
    age: 30,
  ],
  isMember: true,
]
```

Map with Method in Literal Assignment:

```
withMethod val [
  x: 10,
  y: 20,
  move fn dx, dy {
    this.x += dx
    this.y += dy
  }
]
```

#### Type Definition

Record Type Definition:

```
Record<string, number>
```

Array Type Definition:

```
string[]
```

Array Type Definition with Length:

```
number[16]
```

Nested Array Type Definition:

```
number[][]
```

Array of Tuples Type Definition:

```
[number, number][]
```

Tuple Type Definition:

```
[number, string, boolean]
```

Nested Tuple Type Definition:

```
[number, [string, boolean], number]
```

Object Type Definition:

```
[
  x number
  y number
  label string
]
```

Object Type Definition with Optional Fields:

```
[
  x? number
  y? number
  label? string
]
```

Object Type Definition with Methods:

```
[
  x number
  y number
  move fn number, number -> nil
]
```

Nested Object Type Definition:

```
[
  position [
    x number
    y number
  ]
  label string
]
```

#### Pattern Matching

Map Pattern Matching Arm:

```
match myMap {
    x, y, z do todo
}
```

Map Pattern Matching Target:

```
match [x, y] {
  _ do todo
}
```

Map Destructuring with Named Fields:

```
x, y, z = myFunction
```

#### Pattern Matching

Map Pattern Matching Arm:

```
match myArray
  on x, y, z do todo
```

```
match myArray
  on x, .. do todo
```

```
match myArray
  on x, ..rest do todo
```

```
match myArray
  on first, .., last do todo
```

```
match myMap
  on .x, .y, .z do todo
```

```
match myMap
  on  .x 'explicit', .y rename do todo
```

```
match myMap
  on .x, .inner: [.a, .b], .y do todo
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
result val {
  'result'
}
```

Breaking Out of a Block:

```
{
  if true br
  print 'This will never run'
}
```

Saving Block Result with Break Condition to Variable:

```
result val {
  if true br 'Result'
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
result val if true { 'foo' }
           else { 'bar' }
```

## Match

Match Statement with Single Arm:

```
match myValue
  on 1 do print 'One',
```

Match Statement with Multiple Arms:

```
match myValue
  on 1 do print 'One',
  on 2 do print 'Two',
```

Match Statement with Default Arm:

```
match myValue
  on 1 do print 'One',
  on 2 do print 'Two',
  on _ do print 'Other',
```

Match Statement with Pattern Matching:

```
match [x, y]
  on 0, 0 do print 'Both Zero',
  on 1, 1 do print 'Both One',
  on _ do print 'Not the Same',
```

Saving Match Result to Variable:

```
result val
  match myValue
    on true do 'Yes',
    on false do 'No',
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
result val while true {
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

for-in Loop with Map:

```
for item in [1, 2, 3, 4, 5] {
  print item
}
```

```
for value, index in [zero: 0, one: 1, two: 2] {
  print index, value
}
```

for-in Loop with break Condition:

```
for item in array {
  if item == 2 br
  print item
}
```

Saving for-in Loop Result to Variable:

```
result val for item in array {
  if item == 2 br item
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
result val for i var 0, i < 10, inc i {
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
