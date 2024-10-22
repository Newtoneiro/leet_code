class Solution(object):
    def longestConsecutive(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        longest_seq = 0
        nums_set = set(nums)

        for num in nums:
            if num - 1 not in nums_set:
                cur = num
                cur_seq_len = 1
                while cur + 1 in nums_set:
                    cur += 1
                    cur_seq_len += 1

                if cur_seq_len > longest_seq:
                    longest_seq = cur_seq_len

        return longest_seq
