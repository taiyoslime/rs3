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
$ cargo --release run -- examples/1.cnf
(snip)
s SATISFIABLE
v -1 -2 3 4

$ cargo --release run -- examples/2.cnf
(snip)
s UNSATISFIABLE

```

## License
[MIT](LICENSE)
