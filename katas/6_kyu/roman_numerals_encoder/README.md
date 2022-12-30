# Roman numerals encoder
[Codewars kata](https://www.codewars.com/kata/51b62bf6a9c58071c600001b/train/rust)

## Description 
Create a function taking a positive integer as its parameter and returning a string containing the Roman Numeral representation of that integer.

Modern Roman numerals are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero. In Roman numerals 1990 is rendered: 1000=M, 900=CM, 90=XC; resulting in MCMXC. 2008 is written as 2000=MM, 8=VIII; or MMVIII. 1666 uses each Roman symbol in descending order: MDCLXVI.

Example:

```solution(1000); // should return 'M'```

### Encoder cases
Given `int32 num` is:
```
>=1000 -> M
>=900 -> CM
>=500 -> D
>=400 -> CD
>=100 -> C
>=90 -> XC
>=50 -> L
>=40 -> XL
>=10 -> X
>=9 -> IX
>=5 -> V
>=4 -> IV
>=1 -> I

```