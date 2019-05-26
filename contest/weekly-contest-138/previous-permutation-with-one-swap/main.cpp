#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> prevPermOpt1(vector<int>& A) {
        int l = -1;
        for (int i = A.size() - 1; i > 0; --i) {
            if (A[i - 1] > A[i]) {
                l = i;
                break;
            }
        }
        if (l == -1) return A;
        int r = A.size();
        for (int i = l; i < A.size(); ++i) {
            if (A[i] >= A[l - 1]) {
                r = i;
                break;
            }
        }
        swap(A[l - 1], A[r - 1]);
        return A;
    }
};

int main() {
    vector<vector<int>> inputs {
        { 3, 2, 1 },
        { 1, 1, 5 },
        { 1, 9, 4, 6, 7 },
        { 3, 1, 1, 3 },
    };
    for (vector<int> A : inputs) {
        vector<int> ret = Solution().prevPermOpt1(A);
        for (int num : ret) cout << num << " ";
        cout << endl;
    }
}
