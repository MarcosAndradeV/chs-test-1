fn println { print #\n print }

fn empty { // [a] -> bool
    len 0 =
}

fn reduce_rec { // Fn(a e -> a) a [e] -> a
    := xs
    := init
    := f
    (empty xs) if {
        init
    } else {
        (reduce_rec f ((call f) init (head xs)) (tail xs))
    }
}

fn reduce { // Fn(a e -> a) a [e] -> a
    := xs
    := init
    := f
    while (empty xs) ! {
        ((call f) init (head xs)) := init
        (tail xs) := xs
    }
    init
}

fn main {
    (reduce fn {+} 7 []) println
    (reduce fn {+} 7 [1 2]) println
}