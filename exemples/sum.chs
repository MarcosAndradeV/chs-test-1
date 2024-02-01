# Sum all numbers of the array

fn sum {
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
    }
}

@(1 5 8 2)
dup
sum
swap len  # Length
/         # Divide
println