#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> highFive(vector<vector<int>>& items) {
        vector<vector<int>> answer;
        map<int, vector<int>> m;
        for (vector<int> item : items) {
            m[item[0]].push_back(item[1]);
        }
        for (pair<int, vector<int>> p : m) {
            sort(p.second.rbegin(), p.second.rend());
            int sum = 0;
            int i = 0;
            for (int size = p.second.size(); i < size && i < 5; ++i) {
                sum += p.second[i];
            }
            answer.push_back(vector<int> { p.first, sum / i });
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> items {
        { 1, 91 }, { 1, 92 }, { 2, 93 }, { 2, 97 }, { 1, 60 }, { 2, 77 }, { 1, 65 }, { 1, 87 }, { 1, 100 }, { 2, 100 }, { 2, 76 }
    };
    vector<vector<int>> ret = Solution().highFive(items);
    for (vector<int> out : ret) {
        cout << out[0] << ": " << out[1] << endl;
    }
}
