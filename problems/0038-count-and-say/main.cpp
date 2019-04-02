#include <iostream>

using namespace std;

class Solution {
public:
    string countAndSay(int n) {
        string answer = "1";
        while (--n) {
            string s;
            int count = 1;
            char prev = answer[0];
            for (int i = 1, l = answer.length(); i < l; ++i) {
                if (answer[i] != prev) {
                    s += to_string(count) + prev;
                    count = 1;
                } else {
                    ++count;
                }
                prev = answer[i];
            }
            answer = s + to_string(count) + prev;
        }
        return answer;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int n = stringToInteger(line);
        
        string ret = Solution().countAndSay(n);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
