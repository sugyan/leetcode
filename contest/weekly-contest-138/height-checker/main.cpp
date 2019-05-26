#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int heightChecker(vector<int>& heights) {
        vector<int> v(heights);
        sort(v.begin(), v.end());
        int answer = 0;
        for (int i = 0, size = heights.size(); i < size; ++i) {
            if (v[i] != heights[i]) ++answer;
        }
        return answer;
    }
};

int main() {
    vector<int> heights { 1, 1, 4, 2, 1, 3 };
    int ret = Solution().heightChecker(heights);
    cout << ret << endl;
}
