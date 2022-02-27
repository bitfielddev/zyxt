# Zyxt syntax
**Implementations:** Implemented ~~Not implemented~~

## Comments
```
// Single line comment
/* Multi
line
comment */
```
**Implemented:** Lexer ~~Parser Instructor Interpreter~~

## Printing to console
```
#import std;
std.println("hello world");
// in the future it will be:
std.ter.println("hello world");
```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

## Declaring variables
```
int x := 0; // declares it
x = 1; // modifies it
x := 0 // type inference
=== Flags ===
x := 0; // exists within file/class and within package
priv x := 0; // only exists within class (in a class), within file (in a file)
pub x := 0; // exists within file/class, within package, and when imported by another package

const x := 0; // constant, cannot be changed

const priv x := 0; // merge two flags

hoi x := 0; // hoist variable to top of scope


```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

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
list<T>: a mutable array of only T
- [item, ...]
array<T>: an immutable array of only T
- array[item, ...]
tuple<T, ...>: a tuple of T, ...
- tuple[item, ...]
set<T>: a set of only T
- set[item, ...]
fset<T>: an immutable set of only T
- fset[item, ...]
dict<K, V>: a dictionary with K keys and V values
- {key: value, ...}

=== Functions ===
func<tuple<T, ...>, R>: a function that accepts args of T and returns R

Declaring a function:
fn {...} // function that takes in nothing and returns nothing
fn: #A {...} // function that takes in nothing and returns a value
fn|arg: #A, ...| {...} // function that takes in args and returns nothing
fn|arg: #A, ...|: #A {...} // function that takes in args and returns a value
fn|kwarg: #A: 0| {...} // keyword arg
fn|args: #varg<#A>| {...} // variable arguments
fn|kwargs: #vkwarg<#A>| {...} // variable keyword arguments
fn<T>|num: T|: T {...} // generics
fn|num: i32| {...} |string: str| {...} // overloading
ifx|arg1: #A, arg2: #A| {...} // makes function infixable, must be ≥2 args
csr {...} // closure
csrifx|arg1: #A, arg2: #A| {...} // closure infix

Calling a function:
f(); // call function
f(arg) // call function with one argument
f arg // same as above
f(arg: val) // keyword argument
f(arg1, arg2) // call function with two arguments
arg1 f arg2 // if infix

=== Classes ===
cls {...} // normal class
cls|Class| {...} // inherited

=== Enums ===
enum{A, B} // enum

=== Typing ===
T1 | T2: Union
#A: Any
#num: Number
#seq: Sequence
#C<tuple<T, ...>, T>: callable (eg functions, objects)
T?: nullable
```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

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

=== Misc ===
x istype y; // x is of type y
x !istype y; // x is not of type y
x >< y; // swap x and y
x ~ y; // concatenation
&x; // get reference of x
\x; // dereference x

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
99. literals

```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

## Statements
```
=== Conditional ===
if <cond> {...} elif <cond> {...} else {...}
// returns a value

match <var>
case <val> {...}
case <val> {...}
else {...}

while <cond> {...}
do {...} while <cond>;

=== Looping ===
for x in list {...}
for x: int in list {...}

loop {...}

=== Break, Continue & Labels ===
'label loop {
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
```