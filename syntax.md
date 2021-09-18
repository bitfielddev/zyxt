# Zy syntax
**Implementations:** <u>Implemented</u> Not implemented

## Comments
```
// Single line comment
/* Multi
line
comment */
```
**Implemented:** <u>Lexer</u> Parser Instructor Interpreter

## Printing to console
```
#import sys
sys.out.print("hello world");
```
**Implemented:** Lexer Parser Instructor Interpreter

## Declaring variables
```
! int x = 0; // global variable, all scopes
!private int x = 0; // only exists within code block ({})
!fileprivate int x = 0; // only exists within file
!public int x = 0; // exists within file, available when imported by another file
!protected int x = 0; // only exists when class is instantiated

!const int x = 0; // constant, cannot be changed

!const!private int x = 0; // merge two flags
```
**Implemented:** Lexer Parser Instructor Interpreter

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

=== Typing ===
#U<T, ...>: Union
#any: Any
#num: Number (#U<byte, short, int, long, float, double, precise, complex>)
#seq: Sequence (#U<list, array>)
#F<[T, ...], T>: callable (eg functions, objects)
#N<T>: nullable
```
**Implemented:** Lexer Parser Instructor Interpreter