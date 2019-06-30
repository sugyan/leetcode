#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<int> pathInZigZagTree(int label) {
        vector<int> answer { label };
        int n = 1;
        while (n * 2 <= label) n *= 2;
        n /= 2;
        while (true) {
            label = n + n * 2 - 1 - label / 2;
            if (label < 1) break;
            answer.push_back(label);
            n /= 2;
        }
        reverse(answer.begin(), answer.end());
        return answer;
    }
};

int main() {
    vector<int> inputs { 14, 26 };
    for (int label : inputs) {
        vector<int> ret = Solution().pathInZigZagTree(label);
        copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ","));
        cout << endl;
    }
}
