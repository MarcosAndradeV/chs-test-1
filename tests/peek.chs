fn newline {
    "\n" print
}


1 2 peek a b {
    b peek a {
        a print newline
        b print newline
    }
    a print newline
    b print newline
}


[1 2 3] 0 1 peek list idx val {
    list val idx idxset
}

print newline
