#include <bits/stdc++.h>

using namespace std;

// Definition for a Node.
class Node {
   public:
    int val;
    vector<Node *> children;

    Node() {
    }

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node *> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
   public:
    vector<vector<int>> levelOrder(Node *root) {
        vector<vector<int>> answer;
        queue<Node *> q;
        if (root != nullptr) {
            q.push(root);
        }
        while (!q.empty()) {
            int size = q.size();
            vector<int> v;
            for (int i = 0; i < size; ++i) {
                Node *node = q.front();
                v.push_back(node->val);
                for (Node *child : node->children) {
                    q.push(child);
                }
                q.pop();
            }
            answer.push_back(v);
        }
        return answer;
    }
};