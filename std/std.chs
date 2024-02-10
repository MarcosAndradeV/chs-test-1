fn println {
  print "\n" print
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

fn min { # [int]
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        if
        acc
        l idx idxget < {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

fn max { # [int]
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        if
        acc
        l idx idxget > {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

fn sum { # [int] : int
    peek l {
        l len 1 -
        0
        while over 0 >= {
            peek idx acc {
                idx 1 -
                acc l idx idxget
                +
            }
        } swap pop
    }
}

fn find { # [int] int : int
    peek list elem {
        0 while dup list len < {
            if list over idxget
            elem = {
                list len 1 +
                else
                1 +
            }
        } if dup list len > {
            pop
            else
            -1
        }
    }
}