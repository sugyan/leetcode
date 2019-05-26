#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<int> rearrangeBarcodes(vector<int>& barcodes) {
        unordered_map<int, int> um;
        vector<int> answer(barcodes.size());
        for (int num : barcodes) ++um[num];
        vector<pair<int, int>> v;
        while (!um.empty()) {
            int midx, mnum = 0;
            for (pair<int, int> p : um) {
                if (p.second > mnum) {
                    midx = p.first;
                    mnum = p.second;
                }
            }
            v.push_back({ midx, um[midx] });
            um.erase(midx);
        }
        int j = 0;
        for (int i = 0; i < (answer.size() + 1) / 2; ++i) {
            answer[i * 2] = v[j].first;
            if (--v[j].second == 0) ++j;
        }
        for (int i = 0; i < answer.size() / 2; ++i) {
            answer[i * 2 + 1] = v[j].first;
            if (--v[j].second == 0) ++j;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> inputs {
        { 1, 1, 1, 2, 2, 2 },
        { 1, 1, 1, 1, 2, 2, 3, 3 },
    };
    for (vector<int> barcodes : inputs) {
        vector<int> ret = Solution().rearrangeBarcodes(barcodes);
        for (int num : ret) cout << num << " ";
        cout << endl;
    }
}
