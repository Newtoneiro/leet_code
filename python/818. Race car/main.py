class Solution(object):
    def racecar(self, target):
        """
        :type target: int
        :rtype: int
        """
        possibilities = [(0, 1)]
        visited = set([(0, 1)])
        steps = 0
        while True:
            new_possibilities = []
            while len(possibilities) > 0:
                pos, speed = possibilities.pop()
                if pos == target:
                    return steps

                if (pos + speed, speed * 2) not in visited:
                    new_possibilities.append((pos + speed, speed * 2))
                    visited.add((pos + speed, speed * 2))
                if (pos, -1 if speed > 0 else 1) not in visited:
                    new_possibilities.append((pos, -1 if speed > 0 else 1))
                    visited.add((pos, -1 if speed > 0 else 1))

            possibilities = new_possibilities
            steps += 1
