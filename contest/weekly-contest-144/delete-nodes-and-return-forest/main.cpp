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
    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        unordered_set<int> s;
        for (int n : to_delete) s.insert(n);
        vector<TreeNode*> answer;
        if (root != nullptr && s.find(root->val) == s.end()) answer.push_back(root);
        dfs(root, answer, s);
        return answer;
    }
private:
    void dfs(TreeNode* node, vector<TreeNode*>& answer, unordered_set<int>& s) {
        if (node == nullptr) return;
        if (s.find(node->val) != s.end()) {
            if (node->left != nullptr) {
                if (s.find(node->left->val) == s.end()) {
                    answer.push_back(node->left);
                }
                dfs(node->left, answer, s);
            }
            if (node->right != nullptr) {
                if (s.find(node->right->val) == s.end()) {
                    answer.push_back(node->right);
                }
                dfs(node->right, answer, s);
            }
        } else {
            if (node->left != nullptr) {
                bool del = s.find(node->left->val) != s.end();
                dfs(node->left, answer, s);
                if (del) node->left = nullptr;
            }
            if (node->right != nullptr) {
                bool del = s.find(node->right->val) != s.end();
                dfs(node->right, answer, s);
                if (del) node->right = nullptr;
            }
        }
    }
};

void trimLeftTrailingSpaces(string &input) {
    input.erase(input.begin(), find_if(input.begin(), input.end(), [](int ch) {
        return !isspace(ch);
    }));
}

void trimRightTrailingSpaces(string &input) {
    input.erase(find_if(input.rbegin(), input.rend(), [](int ch) {
        return !isspace(ch);
    }).base(), input.end());
}

TreeNode* stringToTreeNode(string input) {
    trimLeftTrailingSpaces(input);
    trimRightTrailingSpaces(input);
    input = input.substr(1, input.length() - 2);
    if (!input.size()) {
        return nullptr;
    }

    string item;
    stringstream ss;
    ss.str(input);

    getline(ss, item, ',');
    TreeNode* root = new TreeNode(stoi(item));
    queue<TreeNode*> nodeQueue;
    nodeQueue.push(root);

    while (true) {
        TreeNode* node = nodeQueue.front();
        nodeQueue.pop();

        if (!getline(ss, item, ',')) {
            break;
        }

        trimLeftTrailingSpaces(item);
        if (item != "null") {
            int leftNumber = stoi(item);
            node->left = new TreeNode(leftNumber);
            nodeQueue.push(node->left);
        }

        if (!getline(ss, item, ',')) {
            break;
        }

        trimLeftTrailingSpaces(item);
        if (item != "null") {
            int rightNumber = stoi(item);
            node->right = new TreeNode(rightNumber);
            nodeQueue.push(node->right);
        }
    }
    return root;
}

string treeNodeToString(TreeNode* root) {
    if (root == nullptr) {
      return "[]";
    }

    string output = "";
    queue<TreeNode*> q;
    q.push(root);
    while(!q.empty()) {
        TreeNode* node = q.front();
        q.pop();

        if (node == nullptr) {
          output += "null, ";
          continue;
        }

        output += to_string(node->val) + ", ";
        q.push(node->left);
        q.push(node->right);
    }
    return "[" + output.substr(0, output.length() - 2) + "]";
}

int main() {
    TreeNode* root = stringToTreeNode("[1,2,3,4,5,6,7]");
    vector<int> to_delete { 3, 5 };
    vector<TreeNode*> ret = Solution().delNodes(root, to_delete);
    for (TreeNode* tree : ret) {
        cout << treeNodeToString(tree) << endl;
    }
}
