#include <bits/stdc++.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {
    }
};

class Codec {
   public:
    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        if (root == nullptr) {
            return "";
        }
        return to_string(root->val) + "," + serialize(root->left) +
               serialize(root->right);
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        istringstream iss(data);
        string s;
        vector<int> v;
        while (getline(iss, s, ',')) {
            v.push_back(stoi(s));
        }
        return to_tree(v, 0, v.size());
    }

   private:
    TreeNode* to_tree(vector<int>& v, int i, int j) {
        if (i == j) {
            return nullptr;
        }
        TreeNode* node = new TreeNode(v[i]);
        int k = i + 1;
        while (k < j && v[k] < v[i]) ++k;
        node->left = to_tree(v, i + 1, k);
        node->right = to_tree(v, k, j);
        return node;
    }
};
