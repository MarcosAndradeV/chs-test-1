@(1 5 8 2)
dup peek l {
    l len 1 - 0
    while peek idx acc { idx acc idx 0 >= } {
        peek idx acc {
            idx 1 - acc l idx idxget +
        }
    } peek idx acc { acc }
}         # Sum
swap len  # Length
/         # Divide
println