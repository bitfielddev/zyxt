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
x = 1; // modifies it
x := 0; // type inference
=== Flags ===
priv x := 0; // only available within module
x := 0; // avaliable within package
pub x := 0; // available to other packages
priv inst x := 0; // instance variable, only available within class/struct
inst x := 0; // instance variable, available within package
pub inst x := // instance variable, available to other packages
// will probs figure out prot some time in the future

const x := 0; // constant, must be known at compile time, cannot be changed

const priv x := 0; // merge two flags


```

## Data types
```
=== Numbers ===
Signed int: i8, i16, i32, i64, i128, isize, ibig (default i32)
Unsigned int: u8, u16, u32, u64, i128, usize, ubig
Floats: f16, f32, f64, (soon f128) (default f64)
Other:
- cpx<T> Complex number of type T
- frac<T> Fraction of type T
- dec Decimal

Examples:
4 // default i32
-6i8
3u64
3.5 // default f64
cpx(5, 7) // cpx<i32>
frac(2, 5) // frac<i32>

=== Booleans & special constants ===
booleans: true, false

=== Strings & Characters ===
str
- "normal string"
- f"formatted string"
- r"raw string"
char
- c"a"
- c"8ac3" // unicode representation

=== Sequences ===
array<T, n>: an immutable array of only T
- [item, ...]
vec<T>: an mutable array of only T
- vec[item, ...]
tu<T, ...>: a tuple of T, ...
- tuple[item, ...]
set<T>: a set of only T
- set[item, ...]
fset<T>: an immutable set of only T
- fset[item, ...]
hmap<K, V>: a hashmap with K keys and V values
- hmap(key: value, ...)

=== Functions ===
func<[T, ...], R>: a function that accepts args of T and returns R

Declaring a function (procedure):
proc {...} // function that takes in nothing and returns nothing
proc: T {...} // function that takes in nothing and returns a value
proc|arg: T, ...| {...} // function that takes in args and returns nothing
proc|arg: T, ...|: T {...} // function that takes in args and returns a value
proc|kwarg: T: 0| {...} // keyword arg
proc|##varg args: vec<T>| {...} // variable arguments
proc|##vkwarg kwargs: hmap<str, T>| {...} // variable keyword arguments
proc<T>|num: T|: T {...} // generics
|arg: T| {...} // also a proc
fn {...} // function without side effects

Calling a function:
f(); // call function
f(arg) or f arg // call function with one argument
f(arg: val) or f arg: val // keyword argument
f(arg1, arg2) or f arg1, arg2 // call function with two arguments

=== Classes ===
class { // class
    ..Class; // inheritance
    x := 3; // static value
    inst value := 3; // instance value with default
    
    _new := fn|&_s, ...| {...}; // class instantiation
    _add := fn|&_s, o: _s@type|: _s@type {_s.value+o.value}; // instance methods
    
    f := fn {...}; // static method
    g := fn |&_s@type, ...| {...}; // classmethod
};
struct | // structs
    x: #num,
    y: #num
| {...}; // same method rules as class

// TODO traits

class_(...) // instantiating class
struct_(...) // instantiating struct

=== Enums ===
enum {
    inst A; // no value
    inst B := 3; // value
    inst C: struct |...|; // struct enum
    inst D: tu<...>; // tuple enum
}
enum_.A; enum_.B; enum_.C(...); enum_.D(...) // instantiating enum

=== Typing ===
T1 / T2: Union (special enum)
_U<T1, T2, ...>: Union
_A: Any
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
x.and y; // and [std.math.bit.and]
x.or y; // or [std.math.bit.or]
x.xor y; // xor [std.math.bit.xor]
x.lsh y; // leftshift [std.math.bit.lsh]
x.rsh y; // rightshift [std.math.bit.rsh]
x.zrsh y; // 0-fill rightshift [std.math.bit.zrsh]

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
T? // option type
x? // ?-unwrap
x?.y; // None if x is None
x ?: y; // y if x is None
x!!; // non-None assertion

T!?E // Result type
x!? // ?-unwrap
x !?: |e| {...}; // calls proc/fn if error in x
x!!; // non-error assertion

=== Misc ===
x >< y; // swap x and y
x ~ y; // concatenation
x @ y; // typecast
&x; // get reference of x
&>x; // get pointer of x
*x; // dereference x
..x; // spread syntax
x |> y // equivalent to y(x)
x |> y(z) // equivalent to y(x, z)
x << y // insertion opr
x >> y // extraction opr

=== Parsing order ===
1. ()
2. foobar() foo.bar() foobar[] foo.bar ()
3. ++ -- + - ! & \
4. @
5. ^
6. ·
7. \* / %
8. × ÷
9. \+ - +- -+
10. < > <= => == != === !== is isnt istype isnttype
11. &&
12. ^^
13. ||
14. ~
15. all assignment operators
16. :=
17. if {}

```

## Keyword expressions
```
=== Conditional ===
if <cond> {...} elif <cond> {...} else {...}
if! ...
// returns a value

match <var>
of <pat> {...}
of <pat> {...}
else {...}

while <cond> {...}
do {...} while <cond>;

=== Return ===
fn'outer {
    fn'inner<T>|num: T|: T {
        ret'outer num;
    }
    ret;
}

=== Preprocess ===
pre {...} // returns a value

=== Defer ===
defer {...} // runs at end of scope
```
