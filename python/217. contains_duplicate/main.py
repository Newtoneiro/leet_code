class Solution(object):
    def containsDuplicate(self, nums):
        """
        :type nums: List[int]
        :rtype: bool
        """
        d = set()
        for elem in nums:
            if elem in d:
                return True
            d.add(elem)
        return False
