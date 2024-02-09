var TARGET := 9;

var nums := [2 7 11 15];

var i := 0;
var j := 0;
while i nums len < {
    0 := j
	while j nums len < {
        if i j != {
            if nums i idxget nums j idxget + TARGET = {
                i print ", " print j print "\n" print
                100 := i
                100 := j
            }
        }
		j 1 + := j
	}
	i 1 + := i
}
