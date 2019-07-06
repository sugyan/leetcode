#include <bits/stdc++.h>

using namespace std;

class KthLargest {
    int k;
    priority_queue<int, vector<int>, greater<int>> pq;
public:
    KthLargest(int k, vector<int>& nums) : k(k) {
        for (int num : nums) {
            pq.push(num);
            if (pq.size() > k) pq.pop();
        }
    }
    int add(int val) {
        pq.push(val);
        if (pq.size() > k) pq.pop();
        return pq.top();
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */

int main() {
    vector<int> nums { 4, 5, 8, 2 };
    KthLargest* obj = new KthLargest(3, nums);
    vector<int> inputs { 3, 5, 10, 9, 4 };
    for (int val : inputs) {
        cout << obj->add(val) << endl;
    }
}
