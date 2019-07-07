#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> corpFlightBookings(vector<vector<int>>& bookings, int n) {
        vector<int> v(n + 1);
        for (vector<int> booking : bookings) {
            v[booking[0] - 1] += booking[2];
            v[booking[1]] -= booking[2];
        }
        vector<int> answer(n);
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            sum += v[i];
            answer[i] = sum;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> bookings {
        { 1, 2, 10 },
        { 2, 3, 20 },
        { 2, 5, 25 },
    };
    vector<int> ret = Solution().corpFlightBookings(bookings, 5);
    copy(ret.begin(), ret.end(), ostream_iterator<int>(cout, ","));
    cout << endl;
}
