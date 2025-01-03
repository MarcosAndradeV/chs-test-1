# **CHS programing language**
An experimental language designed for maximum portability. To write code once and seamlessly execute it across diverse environments.

### **Goals**

* **Develop a Custom Virtual Machine (CHSVM)**
    * To run pure CHASM, make testing and perfomace checks. 
* **Create a Custom Assembly-like Language (CHASM)**
    * To have a common interface to compile to other targets.
* **Implement Backends for:**
    * **C and/or QBE**
    * **WebAssembly (Wasm) and JavaScript**
    * **Java Virtual Machine (JVM)**
    * **Erlang BEAM**

### Example Code

```
use Io

fn main()
  Io.puts("Hello, world")
end

```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References & Inspirations

- Type System [GitHub tomprimozic/type-systems](https://github.com/tomprimozic/type-systems)
- BM: [GitHub - tsoding/bm](https://github.com/tsoding/bm)
- Porth: [GitLab - tsoding/porth](https://gitlab.com/tsoding/porth)
- SmallVM: [GitHub - tarekwiz/smallvm](https://github.com/tarekwiz/smallvm)
- IridiumVM: [GitHub - fhaynes/iridium](https://github.com/fhaynes/iridium)
- Inko: [GitHub - inko-lang/inko](https://github.com/inko-lang/inko)
- Boson-lang: [GitHub - Narasimha1997/boson-lang](https://github.com/Narasimha1997/boson-lang)
- Tao: [GitHub - zesterer/tao](https://github.com/zesterer/tao)
