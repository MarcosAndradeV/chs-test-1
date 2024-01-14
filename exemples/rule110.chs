# Based in this code: https://gist.github.com/rexim/c595009436f87ca076e7c4a2fb92ce10

%def BOARD_SIZE 100
var board List[BOARD_SIZE] ();
set board[BOARD_SIZE 2 -] 1;

var j int 0;
var k int 0;
var pattern int 0;

0 while dup BOARD_SIZE 2 - < {
    set j 0;
    while j BOARD_SIZE < {
        " *"[board[j]] pstr
        set j j 1 +;
    }
    "\n" pstr
    set pattern board[0] 1 << board[1] |;
    set k 0;
    while k BOARD_SIZE 1 - < {
        set pattern pattern 1 << 7 & board[k 1 +] |;
        set board[k] 110 pattern >> 1 &;
        set k k 1 +;
    }
    1 +
}