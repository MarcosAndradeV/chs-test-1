# Pushes a new val to an array

fn push {
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
}

"input: " println
@(1) 2
over println
dup println
"-------" println
push
"output: " println
println
