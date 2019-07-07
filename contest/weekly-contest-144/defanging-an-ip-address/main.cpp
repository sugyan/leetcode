#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string defangIPaddr(string address) {
        string answer;
        for (char c : address) {
            if (c == '.') {
                answer += '[';
                answer += c;
                answer += ']';
            } else {
                answer += c;
            }
        }
        return answer;
    }
};

int main() {
    string address = "1.1.1.1";
    string ret = Solution().defangIPaddr(address);
    cout << ret << endl;
}
