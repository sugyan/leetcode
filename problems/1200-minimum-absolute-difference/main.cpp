#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> minimumAbsDifference(vector<int>& arr) {
        sort(arr.begin(), arr.end());
        vector<vector<int>> answer;
        int m = numeric_limits<int>::max();
        for (int i = 1; i < arr.size(); ++i) {
            int d = arr[i] - arr[i - 1];
            if (d <= m) {
                if (d < m) {
                    answer.clear();
                    m = d;
                }
                answer.push_back({ arr[i - 1], arr[i] });
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> inputs {
        { 4, 2, 1, 3 },
        { 1, 3, 6, 10, 15 },
        { 3, 8, -10, 23, 19, -4, -14, 27 },
    };
    for (vector<int>& arr : inputs) {
        vector<vector<int>> ret = Solution().minimumAbsDifference(arr);
        vector<string> out;
        transform(ret.begin(), ret.end(), back_inserter(out), [](vector<int> cell) {
            return "[" + to_string(cell[0]) + ", " + to_string(cell[1]) + "]";
        });
        copy(out.begin(), out.end(), ostream_iterator<string>(cout, ", "));
        cout << endl;
    }
}
