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
   private:
    unordered_map<Node*, Node*> um;

   public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) return head;
        if (um.find(head) != um.end()) {
            return um[head];
        }
        Node* node = new Node(head->val);
        um[head] = node;
        node->next = copyRandomList(head->next);
        node->random = copyRandomList(head->random);
        return node;
    }
};
