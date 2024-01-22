%def TARGET 9

var nums (2 7 11 15);

0 := i 
while i nums len < {
    0 := j
	while j nums len < {
        if i j != {
            if nums[i] nums[j] + TARGET = {
                i print ", " print j println
                hlt
            }
        }
		j 1 + := j
	}
	i 1 + := i
}