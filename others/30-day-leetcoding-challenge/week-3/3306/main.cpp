#include <bits/stdc++.h>

using namespace std;

// This is the BinaryMatrix's API interface.
// You should not implement it, or speculate about its implementation
class BinaryMatrix {
   public:
    int get(int x, int y);
    vector<int> dimensions();
};

class Solution {
   public:
    int leftMostColumnWithOne(BinaryMatrix &binaryMatrix) {
        vector<int> nm = binaryMatrix.dimensions();
        int answer = nm[1];
        for (int i = 0; i < nm[0]; ++i) {
            if (answer < nm[1] && binaryMatrix.get(i, answer) == 0) {
                continue;
            }
            int l = 0, r = answer;
            while (l < r) {
                int m = l + (r - l) / 2;
                if (binaryMatrix.get(i, m) == 1) {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            answer = l;
        }
        return answer < nm[1] ? answer : -1;
    }
};
