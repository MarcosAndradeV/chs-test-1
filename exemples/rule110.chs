# Based in this code: https://gist.github.com/rexim/c595009436f87ca076e7c4a2fb92ce10

%def BOARD_SIZE 100
var board ();
set board[BOARD_SIZE 1 -] 1;

var i 0;
var pattern 0;

while i BOARD_SIZE 2 - < {
    var j 0;
    while j BOARD_SIZE < {
        " *"[board[j]] print
        set j j 1 +;
    }
    "\n" print
    set pattern board[0] 1 << board[1] |;
    var j 0;
    while j BOARD_SIZE 1 - < {
        set pattern pattern 1 << 7 & board[j 1 +] |;
        set board[j] 110 pattern >> 1 &;
        set j j 1 +;
    }
    set i i 1 +;
}