class Solution(object):
    def threeSum(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """
        nums = sorted(nums)
        out = set()
        for k in range(1, len(nums) - 1):
            i = 0
            j = len(nums) - 1
            while i < k and k < j:
                total = nums[i] + nums[k] + nums[j]

                if total == 0:
                    out.add((nums[i], nums[k], nums[j]))
                    i += 1
                elif total < 0:
                    i += 1
                elif total > 0:
                    j -= 1
        return [list(sol) for sol in out]
