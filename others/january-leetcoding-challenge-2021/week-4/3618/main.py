class Solution:
    def concatenatedBinary(self, n: int) -> int:
        return int("".join([f"{x+1:b}" for x in range(n)]), 2) % (10 ** 9 + 7)
