/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    void dfs(vector<vector<int>>& values, TreeNode* node, int depth) {
        if (node == NULL) return;
        if (values.size() < depth + 1) {
            values.push_back(vector<int>());
        }
        values[depth].push_back(node->val);
        dfs(values, node->left, depth + 1);
        dfs(values, node->right, depth + 1);
    }
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        vector<vector<int>> values;
        dfs(values, root, 0);
        reverse(values.begin(), values.end());
        return values;
    }
};
