from math import ceil


class Solution(object):
    def minEatingSpeed(self, piles, h):
        """
        :type piles: List[int]
        :type h: int
        :rtype: int
        """

        max_bananas = max(piles)
        min_bananas = 1

        def success(k):
            time = sum(ceil(p / k) for p in piles)
            return time <= h

        while min_bananas <= max_bananas:
            mid = min_bananas + (max_bananas - min_bananas) // 2
            if success(mid):
                max_bananas = mid - 1
            else:
                min_bananas = mid + 1
        print(min_bananas, max_bananas)

        return min_bananas
