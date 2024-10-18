from collections import defaultdict


class Solution(object):
    def groupAnagrams(self, strs):
        """
        :type strs: List[str]
        :rtype: List[List[str]]
        """
        out = defaultdict(list)
        for elem in strs:
            out["".join(sorted(elem))].append(elem)

        return out.values()
