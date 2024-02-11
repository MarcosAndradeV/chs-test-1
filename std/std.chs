fn println {
  print "\n" print
}

fn reverse { # [any] : [any]
    if dup len 0 = {
        pop []
        else
        dup tail reverse swap [head] concat
    }
}

fn repeat { # int [any] : [any]
    if over 0 < {
        else
        swap 1 - swap [0] concat repeat
    }
}

fn min { # [int] : int
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

fn max { # [int] : int
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

fn find { # [any] any : int
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