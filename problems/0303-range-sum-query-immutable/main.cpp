#include <bits/stdc++.h>

using namespace std;

class NumArray {
public:
    NumArray(vector<int>& nums) {
        int sum = 0;
        for (int num : nums) {
            sum += num;
            sums.push_back(sum);
        }
    }
    
    int sumRange(int i, int j) {
        return sums[j + 1] - sums[i];
    }
private:
    vector<int> sums = { 0 };
};

int main() {
    vector<int> nums = { -2, 0, 3, -5, 2, -1 };
    NumArray *obj = new NumArray(nums);
    cout << obj->sumRange(0, 2) << endl;
    cout << obj->sumRange(2, 5) << endl;
    cout << obj->sumRange(0, 5) << endl;
}
