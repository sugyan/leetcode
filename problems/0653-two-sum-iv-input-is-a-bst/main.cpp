#include <bits/stdc++.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
//     unordered_set<int> s;
// public:
//     bool findTarget(TreeNode* root, int k) {
//         if (root == nullptr) return false;
//         if (s.find(k - root->val) != s.end()) return true;
//         s.insert(root->val);
//         return findTarget(root->left, k) || findTarget(root->right, k);
//     }
public:
    bool findTarget(TreeNode* root, int k) {
        if (root == nullptr) return false;
        vector<int> nums;
        traverse(root, nums);
        for (int l = 0, r = nums.size() - 1; l < r; ) {
            while (l < r && nums[l] + nums[r] < k) ++l;
            while (l < r && nums[l] + nums[r] > k) --r;
            if (l < r && nums[l] + nums[r] == k) return true;
        }
        return false;
    }
private:
    void traverse(TreeNode* node, vector<int>& nums) {
        if (node == nullptr) return;
        traverse(node->left, nums);
        nums.push_back(node->val);
        traverse(node->right, nums);
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
        int k = stringToInteger(line);
        
        bool ret = Solution().findTarget(root, k);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
