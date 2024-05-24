fn min {
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        acc l idx idxget < if {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

fn max {
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        acc l idx idxget > if {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

[1 2 3] min print "\n" print
[1 2 3] max print "\n" print
