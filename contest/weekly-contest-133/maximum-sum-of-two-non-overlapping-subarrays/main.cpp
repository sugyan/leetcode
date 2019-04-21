#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxSumTwoNoOverlap(vector<int>& A, int L, int M) {
        int answer = 0;
        for (int i = 0; i < A.size() - L - M + 1; ++i) {
            int sum1 = 0, maxsum1 = 0;
            for (int j = 0; j < L; ++j) sum1 += A[j];
            maxsum1 = sum1;
            for (int j = 0; j < i; ++j) {
                sum1 += - A[j] + A[L + j];
                maxsum1 = max(maxsum1, sum1);
            }

            int sum2 = 0, maxsum2 = 0;
            for (int j = 0; j < M; ++j) sum2 += A[L + i + j];
            maxsum2 = sum2;
            for (int j = 0; j < A.size() - L - M - i; ++j) {
                sum2 += - A[L + i + j] + A[L + i + j + M];
                maxsum2 = max(maxsum2, sum2);
            }
            answer = max(answer, maxsum1 + maxsum2);
        }
        for (int i = 0; i < A.size() - L - M + 1; ++i) {
            int sum1 = 0, maxsum1 = 0;
            for (int j = 0; j < M; ++j) sum1 += A[j];
            maxsum1 = sum1;
            for (int j = 0; j < i; ++j) {
                sum1 += - A[j] + A[M + j];
                maxsum1 = max(maxsum1, sum1);
            }

            int sum2 = 0, maxsum2 = 0;
            for (int j = 0; j < L; ++j) sum2 += A[M + i + j];
            maxsum2 = sum2;
            for (int j = 0; j < A.size() - L - M - i; ++j) {
                sum2 += - A[M + i + j] + A[M + i + j + L];
                maxsum2 = max(maxsum2, sum2);
            }
            answer = max(answer, maxsum1 + maxsum2);
        }
        return answer;
    }
};

int main() {
    vector<int> A { 0, 6, 5, 2, 2, 5, 1, 9, 4 };
    int ret = Solution().maxSumTwoNoOverlap(A, 1, 2);
    cout << ret << endl;
}
