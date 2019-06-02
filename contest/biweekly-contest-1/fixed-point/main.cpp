#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int fixedPoint(vector<int>& A) {
        for (int i = 0, size = A.size(); i < size; ++i) {
            if (i == A[i]) {
                return i;
            }
        }
        return -1;
    }
};

int main() {
    vector<int>A { -10, -5, 0, 3, 7 };
    int ret = Solution().fixedPoint(A);
    cout << ret << endl;
}
