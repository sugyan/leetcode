#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<int> distributeCandies(int candies, int num_people) {
        vector<int> answer(num_people, 0);
        int i = 0;
        while (candies > i + 1) {
            answer[i % num_people] += i + 1;
            candies -= i + 1;
            ++i;
        }
        answer[i % num_people] += candies;
        return answer;
    }
};

int main() {
    vector<pair<int, int>> inputs {
        { 7, 4 },
        { 10, 3 },
    };
    for (pair<int, int> p : inputs) {
        vector<int> ret = Solution().distributeCandies(p.first, p.second);
        copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ","));
        cout << endl;
    }
}