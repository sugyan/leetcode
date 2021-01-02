#include <bits/stdc++.h>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {
    }
};

class Solution {
   public:
    TreeNode* getTargetCopy(TreeNode* original, TreeNode* cloned,
                            TreeNode* target) {
        if (cloned == nullptr) {
            return nullptr;
        }
        if (original == target) {
            return cloned;
        }
        TreeNode* l = getTargetCopy(original->left, cloned->left, target);
        if (l != nullptr) {
            return l;
        } else {
            return getTargetCopy(original->right, cloned->right, target);
        }
        return nullptr;
    }
};