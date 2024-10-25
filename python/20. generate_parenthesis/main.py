class Solution(object):
    def generateParenthesis(self, n):
        """
        :type n: int
        :rtype: List[str]
        """
        def gen_possibilities(available, accumulated):
            out = []

            av_opened = available.count("(")
            av_closed = available.count(")")
            opened = accumulated.count("(")
            closed = accumulated.count(")")

            if av_opened > 0 and av_closed > 0:
                out.extend(gen_possibilities(available.replace("(", "", 1), accumulated + "("))
            if av_closed > 0 and opened > closed:
                out.extend(gen_possibilities(available.replace(")", "", 1), accumulated + ")"))
            if av_opened == 0 and av_closed == 0:
                out.append(accumulated)
            
            return out

        available = "".join(["(" for i in range(n)]) + "".join([")" for i in range(n)])

        return gen_possibilities(available, "")
        