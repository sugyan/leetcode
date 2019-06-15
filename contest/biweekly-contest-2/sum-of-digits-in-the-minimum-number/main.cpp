#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int sumOfDigits(vector<int>& A) {
        int m = A[0];
        for (int num : A) m = min(m, num);
        int d = 0;
        while (m > 0) {
            d += m % 10;
            m /= 10; 
        }
        return d % 2 == 0 ? 1 : 0;
    }
};

int main() {
    vector<vector<int>> inputs = {
        { 34, 23, 1, 24, 75, 33, 54, 8 },
        { 99, 77, 33, 66, 55 },
    };
    for (vector<int> A : inputs) {
        int ret = Solution().sumOfDigits(A);
        cout << ret << endl;
    } 
}
