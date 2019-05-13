#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> fizzBuzz(int n) {
        vector<string> answer(n);
        for (int i = 1; i <= n; ++i) {
            string s;
            if (i % 3 == 0) s += "Fizz";
            if (i % 5 == 0) s += "Buzz";
            if (s.empty()) s += to_string(i);
            answer[i - 1] = s;
        }
        return answer;
    }
};

int main() {
    int n = 15;
    vector<string> ret = Solution().fizzBuzz(n);
    for (string out : ret) {
        cout << out << endl;
    }
}
