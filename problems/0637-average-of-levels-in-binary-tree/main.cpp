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
    vector<double> averageOfLevels(TreeNode* root) {
        vector<double> answer;
        if (root == nullptr) return answer;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            long sum = 0;
            int size = q.size();
            for (int i = 0; i < size; ++i) {
                TreeNode* node = q.front();
                q.pop();
                sum += node->val;
                if (node->left != nullptr)  q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            answer.push_back(1.0 * sum / size);
        }
        return answer;
    }
};

int main() {
    TreeNode* root = new TreeNode(3);
    root->left = new TreeNode(9);
    root->right = new TreeNode(20);
    root->right->left = new TreeNode(15);
    root->right->right = new TreeNode(7);
    vector<double> ret = Solution().averageOfLevels(root);
    for (double out : ret) {
        cout << out << endl;
    }
}
