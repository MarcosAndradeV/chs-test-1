# Based in this code: https://gist.github.com/rexim/c595009436f87ca076e7c4a2fb92ce10

%def BOARD_SIZE 100
var board ();
set board[BOARD_SIZE 1 -] 1;

var pattern 0;

0 := i
while i BOARD_SIZE 2 - < {
    0 := j
    while j BOARD_SIZE < {
        " *"[board[j]] print
        j 1 + := j
    }
    "\n" print
    board[0] 1 << board[1] | := pattern
    0 := j
    while j BOARD_SIZE 1 - < {
        pattern 1 << 7 & board[j 1 +] | := pattern
        set board[j] 110 pattern >> 1 &;
        j 1 + := j
    }
    i 1 + := i
}