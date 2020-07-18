This is the manual for the `roll` command line utility.

# Guide

## Constants

When calling the program on a constant number, the number itself will be output.

Example:
```
$ roll 5
5
```

## Classic Rolls

When calling the program using standard notation `MdN`, `M` dice with `N` sides will be rolled and added together.

Note: This is non-deterministic unless the algorithm is seeded! Do not expect repeatable results!

Example:
```
$ roll 2d2
2
$ roll 2d2
2
$ roll 2d2
3
```

Note: This means `Md1` will always output `M`, since a "d1" is a one-sided die, which will always come up 1.

Example:
```
$ roll 2d1
2
$ roll 2d1
2
```

## Arithmetic with Rolls

When calling the program, basic math may be used.

Example:
```
$ roll 2d2+3
6
$ roll 2d2-9
-5
$ roll 2d3 + 1d2
5
```

Note: This process is not sensitive to ordering

## Nested Rolls

It is possible to nest rolls, although it should be noted that this may easily lead to explosion in the memory use of the program, even past the limits of reasonable/acceptable usability.

Example:
```
$ roll (2d2)d2
3
```

Note: The parenthesis are optional, but rolling is left associative. This means `1d2d3` is the same as `(1d2)d3`, not `1d(2d3)`

# Formal Specification

Dice rolls are parsed according to the following [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form):

```
Start = AddLayer ;

AddLayer = [ AddLayer,  AddOperator ], MultLayer ;

MultLayer = [ MultLayer, MultOperator ], Roll ;

Roll = [ Roll, "d" ], SubExpression ;

SubExpression = Number ;
SubExpression = "(", Start, ")" ;

Number = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "%" }+ ;

AddOperator = "+" | "-" ;
MultOperator = "*" | "/" ;
```

Note: Whitespace is ignored by the program when parsing. This means "3 + 2" and "3+2" are identical.

4 * 2d2 * 3
