#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numUniqueEmails(vector<string>& emails) {
        unordered_set<string> us;
        for (string email : emails) {
            string normalized;
            bool local = true;
            for (int i = 0, length = email.length(); i < length; ++i) {
                if (local) {
                    while (email[i] == '.') ++i;
                    if (email[i] == '+') {
                        while (email[i] != '@') ++i;
                    }
                }
                if (email[i] == '@') {
                    local = false;
                }
                normalized += email[i];
            }
            us.insert(normalized);
        }
        return us.size();
    }
};

int main() {
    vector<string> emails {
        "test.email+alex@leetcode.com",
        "test.e.mail+bob.cathy@leetcode.com",
        "testemail+david@lee.tcode.com",
    };
    int ret = Solution().numUniqueEmails(emails);
    cout << ret << endl;
}
