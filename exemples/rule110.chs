// Based in this code: https://gist.github.com/rexim/c595009436f87ca076e7c4a2fb92ce10
// Generates a rule 110

fn repeat { // int [any] : [any]
    (< over 0) if {} else { : 1 - : [0] ++ repeat }
}

BOARD_SIZE := 100;
board := (repeat BOARD_SIZE []);
board (idxset (- BOARD_SIZE 1) 1) := board

pattern := 0;

i := 0;
j := 0;
while (< i (- BOARD_SIZE 2)) {
    0 := j
    while (< j BOARD_SIZE) {
        (idxget " *" (idxget board j)) print
        j 1 + := j
    }
    (print "\n")
    (| (<< (idxget board 0) 1) (idxget board 1)) := pattern
    0 := j
    while (< j (- BOARD_SIZE 1)) {
        (| (& (<< pattern 1) 7) (idxget board (+ j 1))) := pattern
        (idxset board j (& (>> 110 pattern) 1)) := board
        j 1 + := j
    }
    i 1 + := i
}
