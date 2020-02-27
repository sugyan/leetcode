#include <bits/stdc++.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {
    }
};

class Solution {
   public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        vector<TreeNode*> vp, vq;
        dfs(root, p, vp);
        dfs(root, q, vq);
        TreeNode* answer = root;
        for (int i = 0; i < min(vp.size(), vq.size()); ++i) {
            if (vp[i] != vq[i]) return answer;
            answer = vp[i];
        }
        return answer;
    }

   private:
    bool dfs(TreeNode* node, TreeNode* target, vector<TreeNode*>& v) {
        if (node == nullptr) return false;
        v.push_back(node);
        if (node == target) return true;
        if (dfs(node->left, target, v)) {
            return true;
        }
        if (dfs(node->right, target, v)) {
            return true;
        }
        v.pop_back();
        return false;
    }
};
