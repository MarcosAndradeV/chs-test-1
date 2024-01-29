1 2 peek a b {
    b peek a {
        a println
        b println
    }
    a println
    b println
}


@(1 2 3) 1 0 peek list idx val {
    list idx val idxset
}

println