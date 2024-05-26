// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.

TARGET := 9;

nums := [2 7 11 15];

i := 0;
j := 0;
while (< i (len nums)) {
    0 := j
	while (< j (len nums)) {
        (!= i j) if {
            (= (+ (idxget nums i) (idxget nums j)) TARGET) 
            if {
                (print i) (print ", ") (print j) (print "\n")
                (len nums) 1 + := i
                (len nums) 1 + := j
            }
        }
		j 1 + := j
	}
	i 1 + := i
}
