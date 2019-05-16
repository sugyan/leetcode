#include <bits/stdc++.h>

using namespace std;

class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;

    Node() {}

    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};

class Solution {
public:
    Node* construct(vector<vector<int>>& grid) {
        int size = grid.size();
        return construct(grid, 0, 0, size, size);
    }
private:
    Node* construct(vector<vector<int>>& grid, int top, int left, int bottom, int right) {
        bool isLeaf = true;
        bool val = grid[top][left];
        for (int i = top; i < bottom; ++i) {
            for (int j = left; j < right; ++j) {
                if (grid[i][j] != val) {
                    isLeaf = false;
                    break;
                }
                if (!isLeaf) break;
            }
        }
        Node* node = new Node(val, isLeaf, nullptr, nullptr, nullptr, nullptr);
        if (!isLeaf) {
            node->topLeft = construct(grid, top, left, (top + bottom + 1) / 2, (left + right + 1) / 2);
            node->topRight = construct(grid, top, (left + right + 1) / 2, (top + bottom + 1) / 2, right);
            node->bottomLeft = construct(grid, (top + bottom + 1) / 2, left, bottom, (left + right + 1) / 2);
            node->bottomRight = construct(grid, (top + bottom + 1) / 2, (left + right + 1) / 2, bottom, right);
        }
        return node;
    }
};

int main() {
    vector<vector<int>> grid {
        { 1, 1, 1, 1, 0, 0, 0, 0 },
        { 1, 1, 1, 1, 0, 0, 0, 0 },
        { 1, 1, 1, 1, 1, 1, 1, 1 },
        { 1, 1, 1, 1, 1, 1, 1, 1 },
        { 1, 1, 1, 1, 0, 0, 0, 0 },
        { 1, 1, 1, 1, 0, 0, 0, 0 },
        { 1, 1, 1, 1, 0, 0, 0, 0 },
        { 1, 1, 1, 1, 0, 0, 0, 0 },
    };
    Node* ret = Solution().construct(grid);
    stack<pair<Node*, int>> s;
    s.push({ ret, 0 });
    while (!s.empty()) {
        pair<Node*, int> p = s.top();
        s.pop();
        for (int i = 0; i < p.second; ++i) cout << ' ';
        cout << p.first->isLeaf;
        if (p.first->isLeaf) {
            cout << ":" << p.first->val;
        } else {
            s.push({ p.first->topLeft, p.second + 1 });
            s.push({ p.first->topRight, p.second + 1 });
            s.push({ p.first->bottomLeft, p.second + 1 });
            s.push({ p.first->bottomRight, p.second + 1 });
        }
        cout << endl;
    }
}
