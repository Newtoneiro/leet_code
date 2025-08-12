class Solution(object):
    def characterReplacement(self, s, k):
        """
        :type s: str
        :type k: int
        :rtype: int
        """

        cur_substring = s[0]
        max_len = 1
        for ch in s[1:]:
            if cur_substring[-1] == ch:
                cur_substring += ch
            else:
                if k == 0:
                    max_len = max(max_len, len(cur_substring))
