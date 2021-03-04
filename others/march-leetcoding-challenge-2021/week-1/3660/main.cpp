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
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *nodeA = headA, *nodeB = headB;
        while (nodeA != nodeB) {
            nodeA = nodeA != nullptr ? nodeA->next : headB;
            nodeB = nodeB != nullptr ? nodeB->next : headA;
        }
        return nodeA;
    }
};
