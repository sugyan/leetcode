#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> findRestaurant(vector<string>& list1, vector<string>& list2) {
        vector<string> answer;
        unordered_map<string, int> um;
        for (int i = 0, size = list1.size(); i < size; ++i) um[list1[i]] = i;
        int minsum = list1.size() + list2.size();
        for (int j = 0, size = list2.size(); j < size; ++j) {
            if (um.find(list2[j]) != um.end()) {
                int sum = um[list2[j]] + j;
                if (sum < minsum) {
                    answer.clear();
                    minsum = sum;
                }
                if (um[list2[j]] + j == minsum) {
                    answer.push_back(list2[j]);
                }
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<vector<string>>> inputs {
        {
            { "Shogun", "Tapioca Express", "Burger King", "KFC" },
            { "Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun" },
        },
        {
            { "Shogun", "Tapioca Express", "Burger King", "KFC" },
            { "KFC", "Shogun", "Burger King" },
        },
    };
    for (vector<vector<string>> input : inputs) {
        vector<string> ret = Solution().findRestaurant(input[0], input[1]);
        for (string out : ret) cout << out << " ";
        cout << endl;
    }
}
