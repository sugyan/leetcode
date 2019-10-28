#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int distanceBetweenBusStops(vector<int>& distance, int start, int destination) {
        int answer = 0, sum = 0;
        for (int i = 0; i < distance.size(); ++i) {
            if ((i + start) % distance.size() == destination) {
                answer = sum;
            }
            sum += distance[(i + start) % distance.size()];
        }
        return min(answer, sum - answer);
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

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> distance = stringToIntegerVector(line);
        getline(cin, line);
        int start = stringToInteger(line);
        getline(cin, line);
        int destination = stringToInteger(line);
        
        int ret = Solution().distanceBetweenBusStops(distance, start, destination);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
