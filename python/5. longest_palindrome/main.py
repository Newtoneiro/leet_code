class Solution(object):
    def longestPalindrome(self, s):
        """
        :type s: str
        :rtype: str
        """
        longest = s[0]

        for i in range(0, len(s) - 1):
            for k in [i, i+1]:  # i for odd-numbered palindromes, i+1 for even
                for j in range(0, min(i, len(s) - 1 - k) + 1):
                    if s[i - j] == s[k + j]:
                        if len(s[i - j:k+j+1]) > len(longest):
                            longest = s[i - j:k+j+1]
                    else:
                        break

        return longest
