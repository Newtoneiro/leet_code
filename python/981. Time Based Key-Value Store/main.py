class TimeMap(object):

    def __init__(self):
        self.memory = []

    def set(self, key, value, timestamp):
        """
        :type key: str
        :type value: str
        :type timestamp: int
        :rtype: None
        """
        self.memory.append((timestamp, key, value))

    def get(self, key, timestamp):
        """
        :type key: str
        :type timestamp: int
        :rtype: str
        """
        if not self.memory:
            return ""

        left = 0
        right = len(self.memory) - 1
        while left <= right:
            mid = left + (right - left) // 2
            if self.memory[mid][0] == timestamp and self.memory[mid][1] == key:
                return self.memory[mid][2]
            elif self.memory[mid][0] < timestamp:
                left = mid + 1
            else:
                right = mid - 1

        for entry in reversed(self.memory[:left]):
            if entry[1] == key:
                return entry[2]
        return ""


# Your TimeMap object will be instantiated and called as such:
# obj = TimeMap()
# obj.set(key,value,timestamp)
# param_2 = obj.get(key,timestamp)
