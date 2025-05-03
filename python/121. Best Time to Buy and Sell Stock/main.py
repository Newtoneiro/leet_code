class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        buy_price = prices[0]
        profit = 0
        for p in prices[1:]:
            if p < buy_price:
                buy_price = p
                continue
            profit = max(profit, p - buy_price)
        return profit
