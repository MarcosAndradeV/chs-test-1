# Given an array of integers nums and an integer target,
# return indices of the two numbers such that they add up to target.

var TARGET := 9;

var nums := @(2 7 11 15);

var i := 0;
var j := 0;
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
