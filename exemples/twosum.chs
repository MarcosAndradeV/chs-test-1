%def TARGET 9

var nums (2 7 11 15);

var i 0;
while i nums len < {
    var j 0;
	while j nums len < {
        if i j != {
            if nums[i] nums[j] + TARGET = {
                i print ", " print j println
                hlt
            }
        }
		set j j 1 +;
	}
	set i i 1 +;
}