# CHS (WIP)

- [CHS (WIP)](#chs-wip)
  - [Introduction](#introduction)
  - [How to use](#how-to-use)
  - [Using CHS](#using-chs)
  - [Development Milestones](#development-milestones)
  - [License](#license)
  - [References \& Inspirations](#references--inspirations)

## Introduction

CHS is a versatile programing language based on virtual machine written in Rust. It is designed for learning, experimenting.
The syntax resemble [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) but with some c-like elements.

## How to use

(Not stable yet! you may have some troubles.)

```console
git clone git@github.com:MarcosAndradeV/chs-lang.git
cd chs-lang
make chsc
./chsc <file.chs>
```

## Using CHS

- Hello world

```pascal
"Hello, world!\n" print
```

- Variables

```pascal
<name> := <value>;
<value> := <name>
```

```pascal
interger := 10;
20 := interger

text := "Some text";
"Other text" := text
mylist := [1 2 3 4];
```

- if-statmets

```pascal
<condition> if {
  <true-block>
} else {
  <false-block>
}
```

```c
1 1 = if {
  "Everything is fine\n" print
} else {
  "Some thing is wrong\n" print
}
```

- while-loops

```pascal
while <condition> {
  <while-block>
}
```

```c
0 while dup 100 < {
  dup print
  1 +
}
```

- Stack operations

```
dup  |   .   # [a] -> [a a]
pop  | drop  # [a] -> []
swap |   :   # [a b] -> [b a]
over         # [a b] -> [a b a]
rot          # [a b c] -> [b c a]
```

- Logical and Comparison operations

```
=  # equal
!= # not equal
<  # less than
<= # less or equal than
>  # greater than
>= # greater or equal than
|| # logical or
&& # logical and
!  # logical not

```

- Bitwise operations

```
>>  # right bit shift
<<  # left bit shift
|   # bit or
&   # bit and
```

## Development Milestones

- [X] [Turing Complete](exemples/rule110.chs)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References & Inspirations

- BM: [GitHub - tsoding/bm](https://github.com/tsoding/bm)
- Porth: [GitLab - tsoding/porth](https://gitlab.com/tsoding/porth)
- SmallVM: [GitHub - tarekwiz/smallvm](https://github.com/tarekwiz/smallvm)
- IridiumVM: [GitHub - fhaynes/iridium](https://github.com/fhaynes/iridium)
- WebAssembly: [Rc memory management](https://binji.github.io/posts/webassembly-type-checking/)
- Inko: [GitHub - inko-lang/inko](https://github.com/inko-lang/inko)
- Boson-lang: [GitHub - Narasimha1997/boson-lang](https://github.com/Narasimha1997/boson-lang)
