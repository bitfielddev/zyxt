# Zyxt syntax
Note: some my not be implemented yet! 

## Comments
```
// Single line comment
/* Multi
line
comment */
```

## Printing to console
```
ter.out "Hello World";
```

## Declaring variables
```
x: i32 := 0; // declares it
y: i32 .= // declares it, but is mutable
y = 1; // modifies it
x := 0; // type inference
```
### Module Flags
```
x := 0; // avaliable within module
pubp x := 0; // available within package
pub x := 0; // available to other packages
```
### Class Flags
```
inst x := 0; // instance variable, only available within class/struct
pubp inst x := 0; // instance variable, available within package
pub inst x := 0 // instance variable, available to other packages
// will probs figure out prot some time in the future

pub inst x := 0; // merge two flags


```

## Data types
### Numbers
```
Signed int: i8, i16, i32, i64, i128, isize, ibig (default i32)
Unsigned int: u8, u16, u32, u64, i128, usize, ubig
Floats: f16, f32, f64 (default f64)

Examples:
4 // default i32
-6i8
3u64
3.5 // default f64
cpx(5, 7) // cpx<i32>
frac(2, 5) // frac<i32>
```
### Booleans & Unit
```
Booleans (bool): true, false
Unit: (_unit): unit
```
### Strings & Characters
```
str
- "normal string"
- f"formatted string"
- r"raw string"
char
- c"a"
- c"8ac3" // unicode representation

=== Sequences ===
arr[T, n]: an array of only T
- array(item, ...)
vec[T]: an vector of only T
- vec(item, ...)
tup[T, ...]: a tuple of T, ...
- tu(item, ...)
hset[T]: a hashset of only T
- set(item, ...)
hmap[K, V]: a hashmap with K keys and V values
- hmap(key: value, ...)

=== Functions ===
fun[T: array[type], R]: a function that accepts args T and returns R

Declaring a function (procedure):
proc {...} // function that takes in nothing
proc: T {...} // function that takes in nothing and returns a value of type T
proc|arg: T, ...| {...} // function that takes in args
proc|arg: T, ...|: T {...} // function that takes in args and returns a value
proc|kwarg: T: 0| {...} // keyword arg
proc|varg args: vec[T]| {...} // variable arguments
proc|vkwarg kwargs: hmap[str, T]| {...} // variable keyword arguments
proc[T]|num: T|: T {...} // generics
|arg: T| {...} // also a proc
fn {...} // function without side effects

Calling a function:
f(); // call function
f(arg) or f arg // call function with one argument
f(arg: val) or f arg: val // keyword argument
f(arg1, arg2) or f arg1, arg2 // call function with two arguments

=== Classes ===
A := class { // class
    ..Class; // inheritance
    x := 3; // static value
    inst y: i3; // instance value with type
    inst value := 3; // instance value with default
    
    _new := fn|&$, ...| {...}; // class instantiation
    _add := fn|&$, o: $ty|: $ty {$.value+o.value}; // instance methods
    
    f := fn {...}; // static method
};
B := struct | // structs
    x: i32,
    y: i32
| {...}; // same method rules as class

// TODO traits

A(...) // instantiating class
B(...) // instantiating struct

=== Enums ===
C := enum {
    inst A; // no value
    inst B := 3; // value
    inst C: struct {...}; // struct enum
    inst D: tu[...]; // tuple enum
}
C.A; C.B; C.C(...); C.D(...) // instantiating enum

=== Typing ===
T1 / T2: Union (special enum)
_U[T1, T2, ...]: Union
_A: Any
```

## Operators
```
=== Arithmetic ===
x + y; // addition
x - y; // subtraction
x * y; // multiplication (2nd priority)
x / y; // division (1st priority)
x ^ y; // exponent
x % y; // modulo
x++; // increase value of x by 1
x--; // decrease value of x by 1

=== Assignment ===
x = 1; // assignment
x += y; // add y to x
x -= y; // subtract y from x
x *= y; // multiply x by y
x /= y; // divide x by y
x ^= y; // raise x to yth power
x %= y; // x modulo y, store value in x

=== Relational ===
x == y; // equal
x > y; // greater than
x < y; // less than
x >= y; // greater than or equal to
x <= y; // less than or equal to
x != y; // not equal

=== Logical ===
x && y; // and
x || y; // or
!x; // not

=== Option & error handling ===
opt[T] // option type
x? // ?-unwrap
x?.y; // None if x is None
x ?: y; // y if x is None
x!!; // non-None assertion

res[T, R] // Result type
x!? // ?-unwrap
x !?: |e| {...}; // calls proc/fn if error in x
x!!; // non-error assertion

=== Misc ===
x ~ y; // concatenation
x @ y; // typecast
&x; // get reference of x
&>x; // get pointer of x
*x; // dereference x
..x; // spread syntax
x |> y // equivalent to y(x)
x << y // insertion opr into x
x >> y // extraction opr from x
```

## Keyword expressions
```
=== Conditional ===
if <cond> {...} elif <cond> {...} else {...}
// returns a value

match <var>
of <pat> {...}
of <pat> {...}
else {...}

while <cond> {...}
do {...} while <cond>;

=== Return ===
fn'outer {
    fn'inner[T]|num: T|: T {
        ret'outer num;
    }
    ret;
}

=== Preprocess ===
pre {...} // returns a value

=== Defer ===
defer {...} // runs at end of scope
```
