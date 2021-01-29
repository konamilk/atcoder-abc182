# atcoder-abc182

| Problem | Result |
| :--- | :---: |
| A | AC |
| B | AC |
| C | AC |
| D | - |
| E | - |
| F | - |


## C問題
char -> i32
```rust
vec_of_chars
    .into_iter()
    .map(|x| x as i32 - 48)
    .collect::<Vec<i32>>()
```
組み合わせの列挙

```rust
use itertools::Itertools;

let combination = (0..5).combinations(2);   // vec![vec![0,1],vec![0,2]...]
```