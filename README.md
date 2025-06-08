# Leat: 安っぽい関数型言語

動的型付けかつインタプリタ型で安っぽいです

```ocaml
let dec := (\ x . x - 1) in
let fact n :=
  if n == 0
    then 1
    else fact (dec n) * n
in
fact (dec 7)
```
