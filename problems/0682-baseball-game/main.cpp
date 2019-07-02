#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int calPoints(vector<string>& ops) {
        vector<int> v;
        int answer = 0;
        for (string op : ops) {
            if (op == "C") {
                answer -= v.back();
                v.pop_back();
            } else {
                int value = 0;
                if (op == "+") {
                    value = v.back();
                    if (v.size() > 1) {
                        value += *(v.end() - 2);
                    }
                } else if (op == "D") {
                    value = v.back() * 2;
                } else {
                    value = stoi(op);
                }
                answer += value;
                v.push_back(value);
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        { "5", "2", "C", "D", "+" },
        { "5", "-2", "4", "C", "D", "9", "+", "+" },
    };
    for (vector<string> ops : inputs) {
        int ret = Solution().calPoints(ops);
        cout << ret << endl;
    }
}
