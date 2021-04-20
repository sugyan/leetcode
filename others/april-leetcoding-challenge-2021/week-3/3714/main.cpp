#include <bits/stdc++.h>

using namespace std;

// Definition for a Node.
class Node {
   public:
    int val;
    vector<Node*> children;

    Node() {
    }

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
   public:
    vector<int> preorder(Node* root) {
        vector<int> answer;
        dfs(root, answer);
        return answer;
    }

   private:
    void dfs(Node* node, vector<int>& answer) {
        if (node == nullptr) return;
        answer.push_back(node->val);
        for (Node* child : node->children) {
            dfs(child, answer);
        }
    }
};