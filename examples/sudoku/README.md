Programs to convert Sudoku problems to DIMACS CNF format.

The example problems are taken from
- https://en.wikipedia.org/wiki/Sudoku for `1`
- https://www.sudokuwiki.org/Arto_Inkala_Sudoku, http://web.archive.org/web/20120701002640/https://www.telegraph.co.uk/science/science-news/9359579/Worlds-hardest-sudoku-can-you-crack-it.html for `2`, which is regarded as the hardest Sudoku problem in the world

```
$ cat example
9
5 3 0 0 7 0 0 0 0
6 0 0 1 9 5 0 0 0 
0 9 8 0 0 0 0 6 0
8 0 0 0 6 0 0 0 3 
4 0 0 8 0 3 0 0 1 
7 0 0 0 2 0 0 0 6
0 6 0 0 0 0 2 8 0
0 0 0 4 1 9 0 0 5
0 0 0 0 8 0 0 7 9

$ ruby sudoku2cnf.rb < example > example.cnf

$ cargo run --release -- example.cnf | ruby interpret.rb
(snip)
5 3 4 6 7 8 9 1 2
6 7 2 1 9 5 3 4 8
1 9 8 3 4 2 5 6 7
8 5 9 7 6 1 4 2 3
4 2 6 8 5 3 7 9 1
7 1 3 9 2 4 8 5 6
9 6 1 5 3 7 2 8 4
2 8 7 4 1 9 6 3 5

```