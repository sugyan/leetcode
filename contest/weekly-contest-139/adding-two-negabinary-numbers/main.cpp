#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<int> addNegabinary(vector<int>& arr1, vector<int>& arr2) {
        reverse(arr1.begin(), arr1.end());
        reverse(arr2.begin(), arr2.end());
        vector<int> answer(max(arr1.size(), arr2.size()) + 2, 0);
        for (int i = 0, size = answer.size() - 2; i < size; ++i) {
            int b = answer[i];
            if (i < arr1.size()) b += arr1[i];
            if (i < arr2.size()) b += arr2[i];
            answer[i] = b % 2;
            if (b > 1) {
                if (answer[i + 1] == 0) {
                    answer[i + 1] += 1;
                    answer[i + 2] += 1;
                } else {
                    answer[i + 1] = 0;
                }
            }
        }
        reverse(answer.begin(), answer.end());
        while (answer.size() > 1 && answer[0] == 0) {
            answer.erase(answer.begin());
        }
        return answer;
    }
};

int main() {
    vector<int> arr1 { 1, 1, 1, 1, 1 };
    vector<int> arr2 { 1, 0, 1 };
    vector<int> ret = Solution().addNegabinary(arr1, arr2);
    for (int b : ret) cout << b;
    cout << endl;
}
