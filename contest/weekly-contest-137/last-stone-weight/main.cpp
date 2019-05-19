#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        if (stones.size() == 0) return 0;
        if (stones.size() == 1) return stones.front();
        sort(stones.begin(), stones.end());
        int y = stones.back();
        stones.pop_back();
        int x = stones.back();
        stones.pop_back();
        if (y > x) {
            stones.push_back(y - x);
        }
        return lastStoneWeight(stones);
    }
};

int main() {
    vector<int> stones { 2, 7, 4, 1, 8, 1};
    int ret = Solution().lastStoneWeight(stones);
    cout << ret << endl;
}
