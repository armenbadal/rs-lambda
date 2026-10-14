# Լամբդա լեզու և ինտերպրետատոր

_Rust սովորելու համար_

# Շարահյուսությունը

```ebnf
Expression = REAL
    | BOOL
    | IDENT
    | 'lambda' { IDENT } 'is' Expression
    | 'apply' Expression { Expression }
    | 'if' Expression 'then' Expression 'else' Expression
    | 'let' IDENT 'is' Expression { 'and IDENT 'is' Expression } 'in' Expression
    | '(' Expression ')'.
```

# Օրինակներ

## Ֆակտորիալ

```lambda
let
  factorial is lambda n : 
    if (apply eq n 1) then
        1
    else 
        apply mul n (apply factorial (apply sub n 1))
in
  apply factorial 12
```

## Մեծ թիվը

```lambda
let
    max is lambda n m :
        if (apply gt n m) then
            n
        else 
            m
in
    apply max 4 12
```

# Նշումներ

1. Իդենտիֆիկատորների հատուկ տեսակ. `'[' ցանկացած-նիշ ']'`
