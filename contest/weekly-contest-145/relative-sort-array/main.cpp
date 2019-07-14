#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> relativeSortArray(vector<int>& arr1, vector<int>& arr2) {
        map<int, int> m;
        for (int num: arr1) ++m[num];
        vector<int> answer;
        for (int num: arr2) {
            if (m.find(num) != m.end()) {
                for (int i = 0; i < m[num]; ++i) answer.push_back(num);
                m.erase(num);
            }
        }
        for (pair<int, int> p : m) {
            for (int i = 0; i < p.second; ++i) answer.push_back(p.first);
        }
        return answer;
    }
};

int main() {
    vector<int> arr1 { 2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19 };
    vector<int> arr2 { 2, 1, 4, 3, 9, 6 };
    vector<int> ret = Solution().relativeSortArray(arr1, arr2);
    copy(ret.begin(), ret.end(), ostream_iterator<int>(cout, ", "));
    cout << endl;
}
