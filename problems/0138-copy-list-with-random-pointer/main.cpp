#include <bits/stdc++.h>

using namespace std;

// Definition for a Node.
class Node {
   public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};

class Solution {
   public:
    Node* copyRandomList(Node* head) {
        Node* node;
        unordered_map<Node*, Node*> um;
        node = head;
        while (node != nullptr) {
            um[node] = new Node(node->val);
            node = node->next;
        }
        node = head;
        while (node != nullptr) {
            um[node]->next = um[node->next];
            um[node]->random = um[node->random];
            node = node->next;
        }
        return um[head];
    }
};
