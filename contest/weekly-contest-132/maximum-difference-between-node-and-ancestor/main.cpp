#include <bits/stdc++.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    int solve(TreeNode *node, int maximum, int minimum) {
        if (node == nullptr) return 0;
        maximum = max(maximum, node->val);
        minimum = min(minimum, node->val);
        int answer = max(solve(node->left, maximum, minimum), solve(node->right, maximum, minimum));
        return max(answer, max(abs(maximum - node->val), abs(minimum - node->val)));
    }
    int maxAncestorDiff(TreeNode* root) {
        return solve(root, root->val, root->val);
    }
};
