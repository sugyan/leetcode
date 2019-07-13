#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    char nextGreatestLetter(vector<char>& letters, char target) {
        int l = 0, r = letters.size();
        while (l < r) {
            int m = (l + r) / 2;
            if (letters[m] <= target) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return letters[l % letters.size()];
    }
};

int main() {
    vector<char> inputs { 'a', 'c', 'd', 'g', 'j', 'k' };
    for (char target : inputs) {
        vector<char> letters { 'c', 'f', 'j'};
        char ret = Solution().nextGreatestLetter(letters, target);
        cout << ret << endl;
    }
}
