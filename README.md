# CHS programing language

### Goals
  - Static Typed

### Example Code

```
use Io

fn add3(a: int, b: int) -> int =
  add(a, b)

fn main() = do {
    if add(1, 2) < 4  {
      Io:puts("Yes\n")
    } else {
      Io:puts("No\n")
    }
}

```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References & Inspirations

- BM: [GitHub - tsoding/bm](https://github.com/tsoding/bm)
- Porth: [GitLab - tsoding/porth](https://gitlab.com/tsoding/porth)
- SmallVM: [GitHub - tarekwiz/smallvm](https://github.com/tarekwiz/smallvm)
- IridiumVM: [GitHub - fhaynes/iridium](https://github.com/fhaynes/iridium)
- Inko: [GitHub - inko-lang/inko](https://github.com/inko-lang/inko)
- Boson-lang: [GitHub - Narasimha1997/boson-lang](https://github.com/Narasimha1997/boson-lang)
