#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        sort(houses.begin(), houses.end());
        sort(heaters.begin(), heaters.end());
        int answer = 0, hosize = houses.size(), hesize = heaters.size();
        for (int i = 0, j = 0; i < hosize; ++i) {
            while (j + 1 < hesize && heaters[j + 1] < houses[i]) ++j;
            int d = abs(houses[i] - heaters[j]);
            if (j + 1 < hesize) {
                d = min(d, abs(houses[i] - heaters[j + 1]));
            }
            answer = max(answer, d);
        }
        return answer;
    }
};

void trimLeftTrailingSpaces(string &input) {
    input.erase(input.begin(), find_if(input.begin(), input.end(), [](int ch) {
        return !isspace(ch);
    }));
}

void trimRightTrailingSpaces(string &input) {
    input.erase(find_if(input.rbegin(), input.rend(), [](int ch) {
        return !isspace(ch);
    }).base(), input.end());
}

vector<int> stringToIntegerVector(string input) {
    vector<int> output;
    trimLeftTrailingSpaces(input);
    trimRightTrailingSpaces(input);
    input = input.substr(1, input.length() - 2);
    stringstream ss;
    ss.str(input);
    string item;
    char delim = ',';
    while (getline(ss, item, delim)) {
        output.push_back(stoi(item));
    }
    return output;
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> houses = stringToIntegerVector(line);
        getline(cin, line);
        vector<int> heaters = stringToIntegerVector(line);
        
        int ret = Solution().findRadius(houses, heaters);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
