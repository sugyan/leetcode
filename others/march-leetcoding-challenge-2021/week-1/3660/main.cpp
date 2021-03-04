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
        int lengthA = 0, lengthB = 0;
        for (ListNode *node = headA; node != nullptr; node = node->next)
            ++lengthA;
        for (ListNode *node = headB; node != nullptr; node = node->next)
            ++lengthB;

        ListNode *nodeA = headA, *nodeB = headB;
        if (lengthA > lengthB) {
            for (int i = 0; nodeA != nullptr && i < lengthA - lengthB; ++i)
                nodeA = nodeA->next;
        } else {
            for (int i = 0; nodeB != nullptr && i < lengthB - lengthA; ++i)
                nodeB = nodeB->next;
        }

        while (nodeA != nullptr && nodeB != nullptr && nodeA != nodeB) {
            nodeA = nodeA->next;
            nodeB = nodeB->next;
        }
        return nodeA;
    }
};
