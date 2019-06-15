#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int confusingNumberII(int N) {
        vector<int> v;
        while (N > 0) {
            v.push_back(N % 10);
            N /= 10;
        }
        reverse(v.begin(), v.end());
        for (int i = 0, size = v.size(); i < size; ++i) {
            if (v[i] != 0 && v[i] != 1 && v[i] != 6 && v[i] != 8 && v[i] != 9) {
                while (v[i] != 0 && v[i] != 1 && v[i] != 6 && v[i] != 8 && v[i] != 9) {
                    --v[i];
                }
                ++i;
                while (i < size) {
                    v[i++] = 9;
                }
            }
        }
        int answer = 0;
        count(v, 0, 0, answer, true);
        return answer;
    }
private:
    void count(vector<int> v, int n, int digit, int& answer, bool upper) {
        if (digit == v.size()) {
            int l = n;
            int m = 0;
            while (n > 0) {
                int d = (n % 10);
                if (d == 6 || d == 9) d = 15 - d;
                m = m * 10 + d;
                n /= 10;
            }
            if (l != m) {
                ++answer;
            }
            return;
        }
        int start = upper ?  v[digit] : 9;
        for (int i = start; i >= 0; --i) {
            if (i != 0 && i != 1 && i != 6 && i != 8 && i != 9) continue;
            count(v, n * 10 + i, digit + 1, answer, upper && i == v[digit]);
        }
    }
};

int main() {
    vector<int> inputs { 20, 100 };
    for (int N : inputs) {
        int ret = Solution().confusingNumberII(N);
        cout << ret << endl;
    }
}
