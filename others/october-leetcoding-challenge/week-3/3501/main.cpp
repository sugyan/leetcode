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
        if (um.find(node->val) == um.end()) {
            um[node->val] = new Node(node->val);
            for (Node* neighbor : node->neighbors) {
                um[node->val]->neighbors.push_back(cloneGraph(neighbor));
            }
        }
        return um[node->val];
    }

   private:
    unordered_map<int, Node*> um;
};
