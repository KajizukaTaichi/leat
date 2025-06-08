# Leat: 安っぽい関数型言語

動的型付けかつインタプリタ型で安っぽいです

```ocaml
let |> := \ a . \ f . f a in
let fact n :=
  if n == 0
    then 1
    else fact (n - 1) * n
in
5 |> fact
```
