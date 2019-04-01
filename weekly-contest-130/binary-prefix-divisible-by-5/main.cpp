#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& A) {
        vector<bool> answer;
        int n = 0;
        for (vector<int>::iterator it = A.begin(); it != A.end(); ++it) {
            n *= 2;
            if (*it == 1) {
                ++n;
            }
            answer.push_back(n % 5 == 0);
            n %= 5;
        }
        return answer;
    }
};

int main() {
    Solution s = Solution();
    vector<int> v = { 1, 1, 1, 0, 1 };
    vector<bool> answer = s.prefixesDivBy5(v);
    for (vector<bool>::iterator it = answer.begin(); it != answer.end(); ++it) {
        cout << *it << endl;
    }
}
