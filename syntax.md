# Zyxt syntax

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
x = 1; // modifies it
x := 0; // type inference
=== Flags ===
x := 0; // exists within file/class and within package
priv x := 0; // only exists within class (in a class), within file (in a file)
pub x := 0; // exists within file/class, within package, and when imported by another package

const x := 0; // constant, cannot be changed

const priv x := 0; // merge two flags

hoi x := 0; // hoist variable to top of scope


```

## Data types
```
=== Numbers ===
Signed int: i8, i16, i32, i64, i128, isize (default i32)
Unsigned int: u8, u16, u32, u64, i128, usize
Floats: f32, f64 (default f64)
Other:
- cpx<T> Complex number of type T
- frac<T> Fraction of type T

Examples:
4 // default i32
-6i8
3u64
3.5 // default f64
cpx(5, 7) // cpx<i32>
frac(2, 5) // frac<i32>

=== Booleans & special constants ===
booleans: true, false
infinity: inf (any numerical type, default f64)
null type: null
- append ? at end of type to make nullable, eg i32?

=== Strings & Characters ===
str
- "normal string"
- f"formatted string"
- r"raw string"
char
- c"a"
- c"8ac3" // unicode representation

=== Sequences ===
array<T>: an immutable array of only T
- [item, ...]
vec<T>: an mutable array of only T
- vec[item, ...]
tuple<T, ...>: a tuple of T, ...
- tuple[item, ...]
set<T>: a set of only T
- set[item, ...]
fset<T>: an immutable set of only T
- fset[item, ...]
dict<K, V>: a dictionary with K keys and V values
- dict[key: value, ...]

=== Functions ===
func<[T, ...], R>: a function that accepts args of T and returns R

Declaring a function (procedure):
proc {...} // function that takes in nothing and returns nothing
proc: #A {...} // function that takes in nothing and returns a value
proc|arg: #A, ...| {...} // function that takes in args and returns nothing
proc|arg: #A, ...|: #A {...} // function that takes in args and returns a value
proc|kwarg: #A: 0| {...} // keyword arg
proc|args: #varg<#A>| {...} // variable arguments
proc|kwargs: #vkwarg<str, #A>| {...} // variable keyword arguments
proc<T>|num: T|: T {...} // generics
proc|num: i32| {...} |string: str| {...} // overloading
infix proc|arg1: #A, arg2: #A| {...} // makes function infixable, must be ≥2 args
fn {...} // function without side effects
infix fn|arg1: #A, arg2: #A| {...} // closure infix

Calling a function:
f(); // call function
f(arg) or f arg // call function with one argument
f(arg: val) or f arg: val // keyword argument
f(arg1, arg2) or f arg1, arg2 // call function with two arguments
arg1 f arg2 // if infix

=== Classes ===
class { // class
    ..Class; // inheritance
    x := 3; // static value
    inst value := 3; // instance value with default
    
    #init := fn|&#, ...| {...}; // class instantiation
    #add := fn|&#, o: #@type|: #@type {#.value+o.value}; // instance methods
    
    f := fn {...}; // static method
    g := fn |&#cls, ...| {...}; // classmethod
};
struct | // structs
    x: #num,
    y: #num
| {...}; // same method rules as class
mixin {...} // Like classes, but can't be instantiated

class_(...) // instantiating class
struct_(...) // instantiating struct

=== Enums ===
enum {
    A; // no value
    B := 3; // value
    C := struct |...|; // struct enum
}
enum_.A; enum_.B; enum_.C(...) // instantiating enum

=== Typing ===
T1 / T2: Union
#U<T1, T2, ...>: Union
#A: Any
#num: Number
#seq: Sequence
#has_attr(#call: fn<[...], T>): has attribute
#no_inherited<T>: must be T and not an inherited class / struct
T?: nullable
```

## Operators
```
=== Arithmetic ===
x + y; // addition
x - y; // subtraction
x +- y; // additon-subtraction
x ± y; // addition-subtraction
x -+ y; // subtraction-addition
x ∓ y; // subtraction-addition
x · y; // multiplication (1st priority) (can be dot product)
x * y; // multiplication (2nd priority)
x × y; // multiplication (3rd priority) (can be cross product)
x / y; // division (1st priority)
x ÷ y; // division (2nd priority)
x /f y; // division (1st priority) & floor
x /c y; // division (1st priority) & ceiling
x /~ y; // division (1st priority) & round
x ÷f y; // division (2nd priority) & floor
x ÷c y; // division (2nd priority) & ceiling
x ÷~ y; // division (2nd priority) & round
x ^ y; // exponent
x % y; // modulo
x rt y; // root (y^(1/x)) [std.math.rt]
x log y; // logarithm (log y base x) [std.math.log]
x divmod y // divmod  [std.math.divmod]
x++; // increase value of x by 1
x--; // decrease value of x by 1

=== Bitwise ===
compl x; // complement [std.math.bit.compl]
x.compl; // also complement [ditto]
x and y; // and [std.math.bit.and]
x or y; // or [std.math.bit.or]
x xor y; // xor [std.math.bit.xor]
x lsh y; // leftshift [std.math.bit.lsh]
x rsh y; // rightshift [std.math.bit.rsh]
x zrsh y; // 0-fill rightshift [std.math.bit.zrsh]

=== Assignment ===
x = 1; // assignment
x += y; // add y to x
x -= y; // subtract y from x
x *= y; // multiply x by y
x /= y; // divide x by y
x /f= y; // divide x by y, and floor x
x /c= y; // divide x by y, and ceiling x
x /~= y; // divide x by y, and round x
x %= y; // x modulo y, store value in x

=== Relational ===
x == y; // equal
x > y; // greater than
x < y; // less than
x >= y; // greater than or equal to
x <= y; // less than or equal to
x != y; // not equal
x === y; // equal value and type
x !== y; // not equal value or type
x is y; // same reference
x !is y; // different reference

=== Logical ===
x && y; // and
x || y; // or
x ^^ y; // xor
!x; // not

=== Null & error handling ===
x? // becomes #U<x, null>
x?.y; // null if x or y is null
x ?: y; // y if x is null
x!!; // non-null assertion

{...} ?! e :| {...}; // calls proc/fn if error in first block
x !: y; // y if error in x;

=== Misc ===
x istype y; // x is of type y
x !istype y; // x is not of type y
x >< y; // swap x and y
x ~ y; // concatenation
x @ y; // typecast
&x; // get reference of x
\x; // dereference x
**x; // spread syntax
x..y // range syntax

=== Parsing order ===
-1. ()
0. foobar() foo.bar() foobar[] foo.bar ()
1. ++ -- + - ! & \
2. @
3. ^
4. lg rt
5. ·
6. * / /f /c /~ %
7. × ÷ ÷f ÷c ÷~
8. + - +- -+ ± ∓
10. < > <= => == != === !== is isnt istype isnttype
14. &&
15. ^^
16. ||
17. ~
19. ><
20. all assignment operators
21. :=
22. if {}

```

## Keyword expressions
```
=== Conditional ===
if <cond> {...} elif <cond> {...} else {...}
// returns a value

match <var>
of <val> {...}
of <val> {...}
else {...}

while <cond> {...}
do {...} while <cond>;

=== Looping ===
for x in list {...}
for x: int in list {...}
for {init; condition; after_cond} {...}
loop {...}

=== Break, Continue & Labels ===
loop'label {
    ...
    break'label;
    continue'label;
    ...
}

=== Return ===
fn'outer {
    fn'inner<T>|num: T|: T {
        return'outer num;
    }
    return;
}

=== Preprocess ===
pre {...} // returns a value

=== Defer ===
defer {...} // runs at end of scope
```
