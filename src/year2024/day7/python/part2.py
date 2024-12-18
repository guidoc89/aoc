
# lines = open("../test2.txt").read().strip().split("\n")
lines = open("../prod2.txt").read().strip().split("\n")

def is_line_correct(nums: list[int], idx: int, curr_val: int, target: int) -> bool:
    if idx == len(nums):
        return curr_val == target

    add_res = is_line_correct(nums, idx+1, curr_val+nums[idx], target)
    mul_res = is_line_correct(nums, idx+1, curr_val*nums[idx], target)
    concat_res = is_line_correct(nums, idx+1, int(str(curr_val)+str(nums[idx])), target)
    return add_res or mul_res or concat_res

ans = 0
for line in lines: 
    res, nums = line.split(":")
    res = int(res)
    nums = [int(x) for x in nums.split()]

    if is_line_correct(nums, 0, 0, res):
        ans += res


print(ans)
