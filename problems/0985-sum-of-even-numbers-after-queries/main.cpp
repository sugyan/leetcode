#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<int> sumEvenAfterQueries(vector<int>& A, vector<vector<int>>& queries) {
        int sum = 0;
        for (int num : A) {
            if (num % 2 == 0) sum += num;
        }
        vector<int> answer(queries.size());
        int i = 0;
        for (vector<int>& query : queries) {
            if (A[query[1]] % 2 == 0) sum -= A[query[1]];
            A[query[1]] += query[0];
            if (A[query[1]] % 2 == 0) sum += A[query[1]];
            answer[i++] = sum;
        }
        return answer;
    }
};

int main() {
    vector<int> A { 1, 2, 3, 4 };
    vector<vector<int>> queries {
        {  1, 0 },
        { -3, 1 },
        { -4, 0 },
        {  2, 3 },
    };
    vector<int> ret = Solution().sumEvenAfterQueries(A, queries);
    copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ", "));
    cout << endl;
}
