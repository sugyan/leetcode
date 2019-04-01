#include <iostream>

using namespace std;

class Solution {
public:
    string baseNeg2(int N) {
        if (N == 0) {
            return "0";
        }
        string answer = "";
        while (N != 0) {
            if (N % 2 == 0) {
                answer = "0" + answer;
            } else {
                answer = "1" + answer;
                N -= 1;
            }
            N /= -2;
        }
        return answer;
    }
};

int main() {
    Solution s = Solution();
    string answer = s.baseNeg2(100);
    cout << answer << endl;
}
