#include <bits/stdc++.h>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {
    }
};

class Solution {
   public:
    ListNode *detectCycle(ListNode *head) {
        ListNode *node1 = head;
        ListNode *node2 = head;
        while (node2 != nullptr) {
            node1 = node1->next;
            node2 = node2->next;
            if (node2 == nullptr) {
                break;
            }
            node2 = node2->next;
            if (node1 == node2) {
                node1 = head;
                while (node1 != node2) {
                    node1 = node1->next;
                    node2 = node2->next;
                }
                return node1;
            }
        }
        return nullptr;
    }
};
