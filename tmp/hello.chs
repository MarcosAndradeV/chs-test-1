# Based in this code: https://gist.github.com/rexim/c595009436f87ca076e7c4a2fb92ce10

fn repeat {
    if over 0 < {
        
        else
        swap 1 - swap [0] concat repeat
    }
}

fn main {
    var BOARD_SIZE := 10;
    var board := BOARD_SIZE [] repeat;
    board BOARD_SIZE 1 - 1 idxset := board

    var pattern := 0;

    var i := 0;
    var j := 0;
    while i BOARD_SIZE 2 - < {
        0 := j
        while j BOARD_SIZE < {
            " *" board j idxget idxget print
            j 1 + := j
        }
        "\n" print
        board 0 idxget 1 << board 1 idxget | := pattern
        0 := j
        while j BOARD_SIZE 1 - < {
            pattern 1 << 7 & board j 1 + idxget | := pattern
            board j 110 pattern >> 1 & idxset := board
            j 1 + := j
        }
        i 1 + := i
    }
}