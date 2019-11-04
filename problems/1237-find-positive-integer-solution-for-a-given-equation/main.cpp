#include <bits/stdc++.h>

using namespace std;

// This is the custom function interface.
// You should not implement it, or speculate about its implementation
class CustomFunction {
public:
    // Returns f(x, y) for any given positive integers x and y.
    // Note that f(x, y) is increasing with respect to both x and y.
    // i.e. f(x, y) < f(x + 1, y), f(x, y) < f(x, y + 1)
    int f(int x, int y) {
        switch (function_id) {
        case 1:
            return x + y;
        case 2:
            return x * y;
        default:
            break;
        }
        return 0;
    }
    CustomFunction(int id) {
        function_id = id;
    }
private:
    int function_id = 0;
};

class Solution {
public:
    vector<vector<int>> findSolution(CustomFunction& customfunction, int z) {
        vector<vector<int>> answer;
        int y = 1000;
        for (int x = 1; x <= 1000; ++x) {
            while (y > 0 && customfunction.f(x, y) > z) --y;
            if (y < 1) break;
            if (customfunction.f(x, y) == z) {
                answer.push_back({ x, y });
            }
        }
        return answer;
    }
};

int main() {
    vector<pair<CustomFunction, int>> inputs {
        { CustomFunction(1), 5 },
        { CustomFunction(2), 5 },
    };
    for (auto& input : inputs) {
        vector<vector<int>> ret = Solution().findSolution(input.first, input.second);
        vector<string> out;
        transform(ret.begin(), ret.end(), back_inserter(out), [](vector<int> v) {
            return "[" + to_string(v[0]) + ", " + to_string(v[1]) + "]";
        });
        copy(out.begin(), out.end(), ostream_iterator<string>(cout, ", "));
        cout << endl;
    }
}
