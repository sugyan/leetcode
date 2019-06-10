#include <bits/stdc++.h>

using namespace std;

class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
public:
    int maxDepth(Node* root) {
        if (root == nullptr) return 0;
        int ret = 0;
        for (Node* child : root->children) {
            ret = max(ret, maxDepth(child));
        }
        return ret + 1;
    }
};

int main() {
    Node* root = new Node(1, vector<Node*> {
        new Node(3, vector<Node*> {
            new Node(5, vector<Node*> {}),
            new Node(6, vector<Node*> {}),
        }),
        new Node(2, vector<Node*> {}),
        new Node(4, vector<Node*> {}),
    });
    int ret = Solution().maxDepth(root);
    cout << ret << endl;
}
