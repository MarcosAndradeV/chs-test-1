# Pushes a new val to an array

"input: " println
@(1) 1
over println
dup println
"-------" println

peek arr val {
	arr len fill
	0 while dup arr len < {
		peek narr idx {
			narr idx arr idx idxget idxset
			idx 1 +
		}
	} pop
	peek narr {
	  narr narr len 1 - val idxset
	}
}

"output: " println
println
