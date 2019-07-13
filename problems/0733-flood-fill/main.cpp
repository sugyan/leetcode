#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int newColor) {
        int r = image.size(), c = image[0].size();
        dfs(image, sr, sc, r, c, image[sr][sc], newColor);
        return image;
    }
private:
    void dfs(vector<vector<int>>& image, int sr, int sc, int& r, int& c, int color, int& newColor) {
        if (sr < 0 || sr >= r || sc < 0 || sc >= c) return;
        if (image[sr][sc] != color || image[sr][sc] == newColor) return;
        image[sr][sc] = newColor;
        dfs(image, sr - 1, sc, r, c, color, newColor);
        dfs(image, sr, sc - 1, r, c, color, newColor);
        dfs(image, sr + 1, sc, r, c, color, newColor);
        dfs(image, sr, sc + 1, r, c, color, newColor);
    }
};

int main() {
    vector<vector<int>> image {
        { 1, 1, 1 },
        { 1, 1, 0 },
        { 1, 0, 1 },
    };
    int sr = 1;
    int sc = 1;
    int newColor = 2;
    vector<vector<int>> ret = Solution().floodFill(image, sr, sc, newColor);
    for (vector<int> out : ret) {
        copy(out.begin(), out.end(), ostream_iterator<int>(cout, ", "));
        cout << endl;
    }
}
