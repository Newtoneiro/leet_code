class Solution(object):
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        max_len = 0
        sub = ""
        for ch in s:
            if ch in sub:
                sub = sub[sub.index(ch) + 1:]
            sub += ch
            max_len = max(max_len, len(sub))
        return max_len
