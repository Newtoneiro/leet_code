class Solution(object):
    def maxArea(self, height):
        """
        :type height: List[int]
        :rtype: int
        """
        i = 0
        j = len(height) - 1
        max_volume = 0
        while i < j:
            cur_v = (j - i) * min(height[i], height[j])
            max_volume = max(max_volume, cur_v)
            if height[i] <= height[j]:
                i += 1
            else:
                j -= 1
        return max_volume