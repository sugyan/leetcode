#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    void reverseString(vector<char>& s) {
        for (int i = 0, size = s.size(); i < size / 2; ++i) {
            swap(s[i], s[size - 1 - i]);
        }
    }
};

int main() {
    vector<char> s = { 'h', 'e', 'l', 'l', 'o' };
    Solution().reverseString(s);
    cout << string(s.begin(), s.end()) << endl;
}
