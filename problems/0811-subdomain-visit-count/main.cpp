#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<string> subdomainVisits(vector<string>& cpdomains) {
        unordered_map<string, int> um;
        for (string& cpdomain : cpdomains) {
            int i = cpdomain.find_first_of(' ');
            int num = stoi(cpdomain.substr(0, i));
            while (i != -1) {
                um[cpdomain.substr(++i)] += num;
                i = cpdomain.find_first_of('.', i);
            }
        }
        vector<string> answer;
        for (pair<string, int> p : um) {
            answer.push_back(to_string(p.second) + " " + p.first);
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        { "9001 discuss.leetcode.com" },
        { "900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org" },
    };
    for (vector<string>& cpdomains : inputs) {
        vector<string> ret = Solution().subdomainVisits(cpdomains);
        copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ", "));
        cout << endl;
    }
}
