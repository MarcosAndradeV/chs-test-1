fn println { print (print "\n") }

fn to_list { peek x { [x] } }

fn range { // int -> [int]
    := x
    [] := xs
    0 while dup x < {
        dup to_list xs swap ++ := xs
        1 +
    } drop

    xs
}

fn main {
    10 range println
}
