#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxSatisfied(vector<int>& customers, vector<int>& grumpy, int X) {
        int size = customers.size();
        int answer = 0;
        int secret = 0;
        for (int i = 0; i < X; ++i) {
            if (grumpy[i] == 1) secret += customers[i];
        }
        int m = secret;
        for (int i = 0; i < size; ++i) {
            if (grumpy[i] == 0) answer += customers[i];
            if (i + X < size) {
                if (grumpy[i] == 1) secret -= customers[i];
                if (grumpy[i + X] == 1) secret += customers[i + X];
            }
            m = max(m, secret);
        }
        return answer + m;
    }
};

int main() {
    vector<int> customers { 1, 0, 1, 2, 1, 1, 7, 5 };
    vector<int> grumpy    { 0, 1, 0, 1, 0, 1, 0, 1 };
    int X = 3;
    int ret = Solution().maxSatisfied(customers, grumpy, X);
    cout << ret << endl;
}
