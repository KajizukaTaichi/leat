# Leat: 安っぽい関数型言語

- ラムダ計算っぽい動的型付けインタプリタ言語
- 関数は原則1引数1戻り値で、カリー化されている
- 全てが式で文は無く、ループは基本的に再帰を使う
- 名前が記号の関数は中期記法の構文糖になれる
- メタプログラミングで抽象構文木を操作できる

```ocaml
let |> := \a.\f. f a in
let fact n :=
    if n == 0
        then 1
        else fact (n - 1) * n
in
5 |> fact
```

```ocaml
let fizzbuzz n := (
    if n % 15 == 0 then "FizzBuzz"
    else if n % 3 == 0 then "Fizz"
    else if n % 5 == 0 then "Buzz"
    else cast n #string
) in
join (map fizzbuzz (1 ~ 101)) "\n"
```

```ocaml
let to_num n := n (\x. x + 1) 0 in
let inc n := ast-replace n (\x. x) (\x. f x) in
let dec n := ast-replace n (\x. f x) (\x. x) in
let church n :=
    if n == 0 then (\f.\x. x)
    else inc (church (n - 1))
in
let add a b :=
    if to_num b == 0 then a
    else add (inc a) (dec b)
in
to_num (add (church 3) (church 2))
```

**梶塚太智　作**
