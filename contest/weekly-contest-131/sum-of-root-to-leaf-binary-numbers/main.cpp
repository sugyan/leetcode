#include <bits/stdc++.h>

#define DIV 1000000007

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    int sumToLeaf(TreeNode* node, long base) {
        long answer = node->val + base;
        base += node->val;
        long nextbase = base * 2;
        nextbase %= DIV;
        if (node->left) {
            answer += sumToLeaf(node->left, nextbase);
        }
        if (node->right) {
            answer += sumToLeaf(node->right, nextbase);
        }
        if (node->left || node->right) {
            answer -= base;
        }
        return answer % DIV;
    }
    int sumRootToLeaf(TreeNode* root) {
        return sumToLeaf(root, 0);
    }
};
