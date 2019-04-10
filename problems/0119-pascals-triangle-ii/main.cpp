#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> getRow(int rowIndex) {
        long n = 1;
        vector<int> answer { 1 };
        for (int i = 0; i < rowIndex; ++i) {
            n = n * (rowIndex - i) / (i + 1);
            answer.push_back(n);
        }
        return answer;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

string integerVectorToString(vector<int> list, int length = -1) {
    if (length == -1) {
        length = list.size();
    }

    if (length == 0) {
        return "[]";
    }

    string result;
    for(int index = 0; index < length; index++) {
        int number = list[index];
        result += to_string(number) + ", ";
    }
    return "[" + result.substr(0, result.length() - 2) + "]";
}

int main() {
    string line;
    while (getline(cin, line)) {
        int rowIndex = stringToInteger(line);
        
        vector<int> ret = Solution().getRow(rowIndex);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
