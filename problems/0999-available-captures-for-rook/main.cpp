#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numRookCaptures(vector<vector<char>>& board) {
        int answer = 0;
        for (int i = 0; i < 8; ++i) {
            for (int j = 0; j < 8; ++j) {
                if (board[i][j] == 'R') {
                    for (int k = i + 1; k < 8 && board[k][j] != 'B'; ++k) {
                        if (board[k][j] == 'p') {
                            ++answer;
                            break;
                        }
                    }
                    for (int k = i - 1; k >= 0 && board[k][j] != 'B'; --k) {
                        if (board[k][j] == 'p') {
                            ++answer;
                            break;
                        }
                    }
                    for (int k = j + 1; k < 8 && board[i][k] != 'B'; ++k) {
                        if (board[i][k] == 'p') {
                            ++answer;
                            break;
                        }
                    }
                    for (int k = j - 1; k >= 0 && board[i][k] != 'B'; --k) {
                        if (board[i][k] == 'p') {
                            ++answer;
                            break;
                        }
                    }
                }
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<vector<char>>> inputs {
        {
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', '.', '.', 'p', '.', '.', '.', '.' },
            { '.', '.', '.', 'R', '.', '.', '.', 'p' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', '.', '.', 'p', '.', '.', '.', '.' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
        },
        {
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', 'p', 'p', 'p', 'p', 'p', '.', '.' },
            { '.', 'p', 'p', 'B', 'p', 'p', '.', '.' },
            { '.', 'p', 'B', 'R', 'B', 'p', '.', '.' },
            { '.', 'p', 'p', 'B', 'p', 'p', '.', '.' },
            { '.', 'p', 'p', 'p', 'p', 'p', '.', '.' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
            { '.', '.', '.', '.', '.', '.', '.', '.' },
        },
    };
    for (vector<vector<char>>& board : inputs) {
        int ret = Solution().numRookCaptures(board);
        cout << ret << endl;
    }
}
