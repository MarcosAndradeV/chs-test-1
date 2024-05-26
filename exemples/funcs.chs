(print "Hello, world\n")

fn println { print (print "\n") }
fn square  { (* dup) }

fn foreach { : . (!= head nil) if { over over head : call tail : foreach } else { drop drop } }

fn map { : . (!= head nil) if { over over head : [call] : tail rot map ++ } else { drop drop [] } }

list := [2 3 4];

i := 0; 
while (< i (len list)) { 
    list (idxget i) square println 
    (+ i 1) := i 
}

"----" println

(foreach list ~(println square))

"----" println

(map list ~square) println