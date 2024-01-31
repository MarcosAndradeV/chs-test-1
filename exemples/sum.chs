# Sum all numbers of the array

@(1 5 8 2)
dup
peek l {
    l len 1 -
    0
    while over 0 >= {
        peek idx acc {
            idx 1 -
            acc l idx idxget
            +
        }
    } swap pop
}         # Sum
swap len  # Length
/         # Divide
println