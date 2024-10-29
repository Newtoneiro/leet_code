class Solution(object):
    def largestRectangleArea(self, heights):
        """
        :type heights: List[int]
        :rtype: int
        """
        max_area = 0
        rectangles = set([(1, heights[0])])

        for h in heights[1:]:
            if len(rectangles) == 0:
                rectangles.append((1, h))

            new_rectangles = set()
            for r in rectangles:
                if h == r[1]:
                    new_rectangles.add((r[0] + 1, r[1]))
                elif h > r[1]:
                    new_rectangles.add((r[0] + 1, r[1]))
                    new_rectangles.add((1, h))
                elif h < r[1]:
                    max_area = max(max_area, r[0] * r[1])
                    new_rectangles.add((r[0] + 1, h))

            rectangles = new_rectangles

            reduced = {}
            for left, right in rectangles:
                if right not in reduced or left > reduced[right]:
                    reduced[right] = left
            rectangles = set((left, right) for right, left in reduced.items())

        for r in rectangles:
            max_area = max(max_area, r[0] * r[1])

        return max_area
