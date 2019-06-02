#include <iostream>

using namespace std;

class Solution {
public:
    string gcdOfStrings(string str1, string str2) {
        int l1 = str1.length(), l2 = str2.length();
        int g = gcd(l1, l2);
        for (int i = g; i > 0; --i) {
            if (g % i != 0) continue;
            string s = str1.substr(0, i);
            bool ok = true;
            for (int j = 0; j < l1 / i; ++j) {
                if (str1.substr(j * i, i) != s) {
                    ok = false;
                    break;
                }
            }
            for (int j = 0; j < l2 / i; ++j) {
                if (str2.substr(j * i, i) != s) {
                    ok = false;
                    break;
                }
            }
            if (ok) return s;
        }
        return "";
    }
private:
    int gcd(int a, int b) {
        return a == 0 ? b : gcd(b % a, a);
    }
};

int main() {
    string str1 = "ABCABC";
    string str2 = "ABC";
    string ret = Solution().gcdOfStrings(str1, str2);
    cout << ret << endl;
}
