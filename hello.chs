fn println { (>= $ 1) if {print (print "\n")} }

fn foreach { : . (!= head nil) if { over over head : call tail : foreach } else { drop drop }}

fn map { (>= $ 2) ! if { error "map" } : . (!= head nil) if { over over head : [call] : tail rot map ++ } else { drop drop [] } }

fn foo {
    1 2 peek a b {
        a println
        b println
    }
}

list := [2 3 4];

list ~(println (+ 1)) foreach
(map list ~(+ 1))  println
