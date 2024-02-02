fn min {
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
    }
  }
}

fn max {
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
    }
  }
}

@(1 2 3) min println
@(1 2 3) max println
