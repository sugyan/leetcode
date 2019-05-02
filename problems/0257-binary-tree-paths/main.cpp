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
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> answer;
        if (root == nullptr) return answer;
        if (root->left == nullptr && root->right == nullptr) {
            answer.push_back(to_string(root->val));
        }
        if (root->left != nullptr) {
            for (string s : binaryTreePaths(root->left)) {
                answer.push_back(to_string(root->val) + "->" + s);
            }
        }
        if (root->right != nullptr) {
            for (string s : binaryTreePaths(root->right)) {
                answer.push_back(to_string(root->val) + "->" + s);
            }
        }
        return answer;
    }
};
