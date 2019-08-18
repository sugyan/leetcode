#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int robotSim(vector<int>& commands, vector<vector<int>>& obstacles) {
        unordered_map<int, vector<int>> ox, oy;
        for (vector<int> obstacle : obstacles) {
            ox[obstacle[0]].push_back(obstacle[1]);
            oy[obstacle[1]].push_back(obstacle[0]);
        }
        pair<int, int> p { 0, 0 };
        pair<int, int> d { 0, 1 };
        int answer = 0;
        for (int command : commands) {
            if (command == -2) {
                d = { -d.second, d.first };
            } else if (command == -1) {
                d = { d.second, -d.first };
            } else {
                if (d.first == 0) {
                    int a = p.second + d.second * command;
                    if (d.second > 0) {
                        for (int y : ox[p.first]) {
                            if (y > p.second) a = min(y - 1, a);
                        }
                    } else {
                        for (int y : ox[p.first]) {
                            if (y < p.second) a = max(y + 1, a);
                        }
                    }
                    p.second = a;
                } else {
                    int a = p.first + d.first * command;
                    if (d.first > 0) {
                        for (int x : oy[p.second]) {
                            if (x > p.first) a = min(x - 1, a);
                        }
                    } else {
                        for (int x : oy[p.second]) {
                            if (x < p.first) a = max(x + 1, a);
                        }
                    }
                    p.first = a;
                }
            }
            answer = max(answer, p.first * p.first + p.second * p.second);
        }
        return answer;
    }
};

int main() {
    vector<pair<vector<int>, vector<vector<int>>>> inputs {
        {
            { 4, -1, 3 },
            {
            },
        },
        {
            { 4, -1, 4, -2, 4 },
            {
                { 2, 4 },
            },
        },
    };
    for (pair<vector<int>, vector<vector<int>>> input : inputs) {
        int ret = Solution().robotSim(input.first, input.second);
        cout << ret << endl;
    }
}
