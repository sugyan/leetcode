#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int twoCitySchedCost(vector<vector<int>>& costs) {
        int answer = 0;
        int A = 0, B = 0;
        vector<int> va, vb;
        for (vector<int> v : costs) {
            if (v.front() > v.back()) {
                ++B;
                answer += v.back();
                vb.push_back(v.front() - v.back());
            } else {
                ++A;
                answer += v.front();
                va.push_back(v.back() - v.front());

            }
        }
        if (A > B) {
            sort(va.begin(), va.end());
            for (int i = 0; i < (A - B) / 2; ++i) {
                answer += va[i];
            }
        }
        if (B > A) {
            sort(vb.begin(), vb.end());
            for (int i = 0; i < (B - A) / 2; ++i) {
                answer += vb[i];
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> costs { { 10, 20 }, { 30, 200 }, { 400, 50 }, { 30, 20 } };
    auto ret = Solution().twoCitySchedCost(costs);
    cout << ret << endl;
}
