#include <bits/stdc++.h>

using namespace std;

// Definition for a Node.
class Node {
   public:
    int val;
    Node* prev;
    Node* next;
    Node* child;
};

class Solution {
   public:
    Node* flatten(Node* head) {
        for (Node* node = head; node != nullptr; node = node->next) {
            if (node->child != nullptr) {
                Node* next = node->next;
                Node* children = flatten(node->child);
                node->next = children;
                node->child = nullptr;
                children->prev = node;
                Node* c = children;
                while (c->next != nullptr) c = c->next;
                c->next = next;
                if (next != nullptr) next->prev = c;
            }
        }
        return head;
    }
};
