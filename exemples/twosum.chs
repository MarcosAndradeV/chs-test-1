%def TARGET 9
%def NUMS_LEN 4

# 
var nums List[NUMS_LEN] (2 7 11 15);

var i int 0;
while i NUMS_LEN < {
    var j int 0;
	while j NUMS_LEN < {
        i j != if {
            nums[i] nums[j] + TARGET = if {
                i pstr ", " pstr j pstr "\n" pstr
                hlt
            }
        }
		set j j 1 +;
	}
	set i i 1 +;
}