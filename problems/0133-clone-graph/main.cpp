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
};

class Solution {
   public:
    Node* cloneGraph(Node* node) {
        if (node == nullptr) {
            return node;
        }
        unordered_map<int, vector<int>> um;
        deque<Node*> d;
        d.push_back(node);
        while (!d.empty()) {
            Node* f = d.front();
            d.pop_front();
            um[f->val] = vector<int>();
            for (Node* n : f->neighbors) {
                um[f->val].push_back(n->val);
                if (um.find(n->val) == um.end()) {
                    d.push_back(n);
                    um[n->val] = vector<int>();
                }
            }
        }
        unordered_map<int, Node*> nodes;
        for (pair<int, vector<int>> p : um) {
            if (nodes.find(p.first) == nodes.end()) {
                nodes[p.first] = new Node(p.first);
            }
            for (int n : p.second) {
                if (nodes.find(n) == nodes.end()) {
                    nodes[n] = new Node(n);
                }
                nodes[p.first]->neighbors.push_back(nodes[n]);
            }
        }
        return nodes[1];
    }
};
