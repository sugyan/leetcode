#include <bits/stdc++.h>

using namespace std;

// Definition for a Node.
class Node {
   public:
    int val;
    vector<Node*> neighbors;

    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }

    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }

    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
   public:
    Node* cloneGraph(Node* node) {
        if (node == nullptr) {
            return nullptr;
        }
        if (um.find(node->val) != um.end()) {
            return um[node->val];
        }
        Node* ret = new Node(node->val);
        um[node->val] = ret;
        for (Node* neighbor : node->neighbors) {
            ret->neighbors.push_back(cloneGraph(neighbor));
        }
        return ret;
    }

   private:
    unordered_map<int, Node*> um;
};
