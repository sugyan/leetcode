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
    TreeNode* mergeTrees(TreeNode* t1, TreeNode* t2) {
        // if (t1 == nullptr) return t2;
        // if (t2 == nullptr) return t1;
        // t1->val += t2->val;
        // t1->left = mergeTrees(t1->left, t2->left);
        // t1->right = mergeTrees(t1->right, t2->right);
        // return t1;
        if (t1 == nullptr) return t2;
        stack<pair<TreeNode*, TreeNode*>> s;
        s.push({ t1, t2 });
        while (!s.empty()) {
            pair<TreeNode*, TreeNode*> p = s.top();
            s.pop();
            if (p.first == nullptr || p.second == nullptr) continue;
            p.first->val += p.second->val;
            if (p.first->left != nullptr) {
                s.push({ p.first->left, p.second->left });
            } else {
                p.first->left = p.second->left;
            }
            if (p.first->right != nullptr) {
                s.push({ p.first->right, p.second->right });
            } else {
                p.first->right = p.second->right;
            }
        }
        return t1;
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
    string line;
    while (getline(cin, line)) {
        TreeNode* t1 = stringToTreeNode(line);
        getline(cin, line);
        TreeNode* t2 = stringToTreeNode(line);
        
        TreeNode* ret = Solution().mergeTrees(t1, t2);

        string out = treeNodeToString(ret);
        cout << out << endl;
    }
    return 0;
}
