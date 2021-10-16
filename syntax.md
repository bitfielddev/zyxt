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
#import sys
sys.out.print("hello world");
```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

## Declaring variables
```
=== Flags ===
int x = 0; // global variable, all scopes, only exists within file
priv int x = 0; // only exists within code block ({})
pub int x = 0; // exists within file, available when imported by another file
prot int x = 0; // only exists when class is instantiated

const int x = 0; // constant, cannot be changed

const priv int x = 0; // merge two flags

hoi int x = 0; // hoist variable

=== Numbers ===
byte x = 1b;
short x = 1s;
int x = 1;
long x = 1l;
float x = 1.0f;
double x = 1.0;
precise x = 1.0p;
complex<int> x = c[1, 2];

=== Booleans ===
bool x = true;
bool x = false;

=== Strings & Characters ===
str x = "foobar";
char x = 'a';

=== Functions ===
func f = || void {...}
func f = |str x, int y| str {...}
func f = |str x, int y=2| str {...}
func f = |str *x| str {...}
func f = |str **x| str {...}

func f = |#U[str, char] x| #U[str, char] {...}
func f = |#any x| void {...}
func f = |#num x| void {...}
func f = |#seq x| void {...}
func f = |#F<[int, int], int> x| void {...}
func f = |#N<int> x| void {...}

=== Classes & Enumerators ===
class c = () {...}
class c = (inheritedClass) {...}
enum e = e[..., ...];

=== Lists, Arrays & Dicts ===
list l = l[..., ...];
array a = a[..., ...];
dict d = d[a: b, c: d, ...];

```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~

## Data types
```
byte: 1 byte integer (-128 to 127)
short: 2 byte integer (-32,768 to 32,767)
int: 4 byte integer (-2,147,483,648 to 2,147,483,647)
long: 8 byte integer (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
float: 4 byte floating-point number (6-7dp)
double: 8 byte floating-point number (15dp)
precise: precise number (-inf to inf)
complex<T>: a complex number of T
bool: boolean
str: string
char: character
list: a mutable array
list<T>: a mutable array of only T
array: an immutable array
array<T>: an immutable array of only T
dict: a dictionary
dict<T, T>: a dictionary with T keys and T values

func: a function
class: a class
enum: an enumerator

null: null
void: void, for functions

=== Typing ===
#U<T, ...>: Union
#any: Any
#num: Number (#U<byte, short, int, long, float, double, precise, complex>)
#seq: Sequence (#U<list, array>)
#F<[T, ...], T>: callable (eg functions, objects)
#N<T>: nullable
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
x rt y; // root (y^(1/x))
x lg y; // logarithm (log y base x)
x++; // increase value of x by 1
x--; // decrease value of x by 1

=== Bitwise ===
b~ x; // complement
x b& y; // and
x b| y; // or
x b^ y; // xor
x b<< y; // leftshift
x b>> y; // rightshift
x b>>> y; // 0-fill rightshift

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
x b&= y; // bitwise and with y
x b|= y; // bitwise or with y
x b^= y; // bitwise xor with y
x b<<= y; // bitwise leftshift by y
x b>>= y; // bitwise rightshift by y
x b>>>= y; // bitwise 0-fill rightshift by y

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
x isnt y; // different reference

=== Logical ===
x && y; // and
x || y; // or
x ^^ y; // xor
!x; // not

=== Misc ===
x ? a : b; // ternary operator
x istype y; // x is of type y
x isnttype y; // x is not of type y
x >< y; // swap x and y
x .. y; // concatenation
```
**Implemented:** ~~Lexer Parser Instructor Interpreter~~