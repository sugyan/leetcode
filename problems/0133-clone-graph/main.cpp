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
            return node;
        }
        unordered_map<Node*, Node*> um;
        return cloneGraph(node, um);
    }

   private:
    Node* cloneGraph(Node* node, unordered_map<Node*, Node*>& memo) {
        if (memo.find(node) == memo.end()) {
            memo[node] = new Node(node->val);
            for (Node* neighbor : node->neighbors) {
                memo[node]->neighbors.push_back(cloneGraph(neighbor, memo));
            }
        }
        return memo[node];
    }
};
