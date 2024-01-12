mem 0 store

1 while dup 1000 < {
    dup 3 mod 0 = 
    over 5 mod 0 = | if {
        dup mem load + mem swap store
    }
    1 +
}

mem load print