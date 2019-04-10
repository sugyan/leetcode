#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> answer;
        if (numRows > 0) answer.push_back(vector<int> { 1 });
        for (int i = 1; i < numRows; ++i) {
            vector<int> v { 1 };
            for (int j = 0, l = answer[i - 1].size(); j < l - 1; ++j) {
                v.push_back(answer[i - 1][j] + answer[i - 1][j + 1]);
            }
            v.push_back(1);
            answer.push_back(v);
        }
        return answer;
    }
};
