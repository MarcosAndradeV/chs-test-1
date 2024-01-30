var TARGET := 9;

var nums := @(2 7 11 15);

0 := i 
while i nums len < {
    0 := j
	while j nums len < {
        if i j != {
            if nums i idxget nums j idxget + TARGET = {
                i print ", " print j println
                100 := i
                100 := j
            }
        }
		j 1 + := j
	}
	i 1 + := i
}