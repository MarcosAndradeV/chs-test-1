# Given an array of integers nums and an integer target,
# return indices of the two numbers such that they add up to target.

fn main()
    TARGET := 9
    nums := [2 7 11 15]
    twosun(TARGET, nums)
end

fn twosum(target: int, nums: [int]) -> (int, int)
    l := len(nums)
    i := 0
    while( i < l) {
        j := 0
    	while(j < l) {
            if(i != j && nums[i] + nums[j] == target)
                return { i, j }
            end
    		set j = j + 1
    	end
    	set i = i + 1
    end
end
