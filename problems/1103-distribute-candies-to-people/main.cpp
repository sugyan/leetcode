#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> distributeCandies(int candies, int num_people) {
        vector<int> answer(num_people, 0);
        for (int i = 0; candies > 0; ++i) {
            answer[i % num_people] += min(i + 1, candies);
            candies -= i + 1;
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
        int candies = stringToInteger(line);
        getline(cin, line);
        int num_people = stringToInteger(line);
        
        vector<int> ret = Solution().distributeCandies(candies, num_people);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
