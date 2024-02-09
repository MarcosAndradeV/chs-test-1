fn length { # [int] : int
    if dup len 0 = {
        pop 0
        else
        dup tail len 1 +
    }
    swap pop
}

fn reverse { # [int] : [int]
    if dup len 0 = {
        pop []
        else
        dup tail reverse swap head peek h { [0] 0 h idxset } concat
    }
}

fn repeat { # int [int] : [int]
    if over 0 < {
        
        else
        swap 1 - swap [0] concat repeat
    }
}