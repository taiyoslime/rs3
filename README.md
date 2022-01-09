# [WIP] rs3
A toy [DPLL](https://en.wikipedia.org/wiki/DPLL_algorithm) based SAT solver written in Rust. 
## Build
```
$ git clone https://github.com/taiyoslime/rs3
$ cd rs3
$ cargo build --release
```

## Usage(from CLI)
supports [DIMACS CNF format](http://www.domagoj-babic.com/uploads/ResearchProjects/Spear/dimacs-cnf.pdf) as input.
```
$ cd target/release
$ ./rs3 examples/1.cnf
s SATISFIABLE
v -1 -2 3 4

$ ./rs3 examples/2.cnf
s UNSATISFIABLE

```


## License
[MIT](LICENSE)
