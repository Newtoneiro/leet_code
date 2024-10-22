class Solution(object):
    def isValidSudoku(self, board):
        """
        :type board: List[List[str]]
        :rtype: bool
        """
        def check_for_duplicates(lst):
            for elem in lst:
                if elem != "." and lst.count(elem) > 1:
                    return False
            return True

        for row in board:
            if not check_for_duplicates(row):
                return False

        for i in range(9):
            col = [row[i] for row in board]
            if not check_for_duplicates(col):
                return False

        for i in range(3, 10, 3):
            for j in range(3, 10, 3):
                square = [
                        row[x] 
                        for row in board[j-3:j]
                        for x in range(i-3, i)
                    ]
                if not check_for_duplicates(square):
                    return False

        return True
