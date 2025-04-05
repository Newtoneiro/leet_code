class Solution(object):
    def divide(self, dividend, divisor):
        """
        :type dividend: int
        :type divisor: int
        :rtype: int
        """
        sign = not ((dividend >= 0) ^ (divisor >= 0))
        if abs(divisor) == 1:
            return abs(dividend) if sign else -abs(dividend)

        quotient = 0
        remainer = abs(dividend)
        while remainer - abs(divisor) >= 0:
            remainer -= abs(divisor)
            quotient += 1

        return quotient if sign else -quotient
