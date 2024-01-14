%def TARGET 9
%def NUMS_LEN 4

# [2,7,11,15]
var nums ptr 4
nums 0 +   2 store
nums 1 +   7 store
nums 2 +  11 store
nums 3 +  15 store

nums while dup NUMS_LEN < {
	nums while dup NUMS_LEN < {
        dup2 = if { else
            dup2 load swap load + TARGET = if {
                swap pstr ", " pstr pstr "\n" pstr hlt
            }
        }
		1 +
	} pop
	1 +
} pop