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
    bool isCousins(TreeNode* root, int x, int y) {
        queue<pair<TreeNode*, TreeNode*>> q;
        q.push({ root, nullptr });
        q.push({ nullptr, nullptr });
        int depth = 0;
        pair<TreeNode*, int> dx, dy;
        while (true) {
            pair<TreeNode*, TreeNode*> p = q.front();
            q.pop();
            if (p.first == nullptr) {
                ++depth;
                q.push({ nullptr, nullptr });
                p = q.front();
                q.pop();
            }
            if (p.first == nullptr) break;
            if (p.first->val == x) dx = { p.second, depth };
            if (p.first->val == y) dy = { p.second, depth };
            if (p.first->left != nullptr) q.push({ p.first->left, p.first });
            if (p.first->right != nullptr) q.push({ p.first->right, p.first });
        }
        return dx.first != dy.first && dx.second == dy.second;
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

int stringToInteger(string input) {
    return stoi(input);
}

string boolToString(bool input) {
    return input ? "True" : "False";
}

int main() {
    string line;
    while (getline(cin, line)) {
        TreeNode* root = stringToTreeNode(line);
        getline(cin, line);
        int x = stringToInteger(line);
        getline(cin, line);
        int y = stringToInteger(line);
        
        bool ret = Solution().isCousins(root, x, y);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
