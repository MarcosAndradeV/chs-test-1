(print "Hello, world\n")

fn println { print (print "\n") }
fn square  { (* dup) }

fn foreach { : . (!= head nil) if { over over head : call tail : foreach } else { drop drop } }

fn map { : . (!= head nil) if { over over head : [call] : tail rot map ++ } else { drop drop [] } }

[2 3 4] := list

0 := i
while (< i (len list)) { 
    list (idxget i) square println 
    (+ i 1) := i 
}

"----" println

(foreach list {square println})

"----" println

(map list {square}) println