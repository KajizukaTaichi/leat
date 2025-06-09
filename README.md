# Leat: 安っぽい関数型言語

ラムダ計算ベース。
関数は1引数1戻り値で、カリー化されている。
全てが式で、文は無い。ループは再帰を使う。
名前が記号な関数は、中期記法の構文糖がある。

```ocaml
let |> := \a.\f. f a in
let fact n :=
  if n == 0
    then 1
    else fact (n - 1) * n
in
5 |> fact
```
