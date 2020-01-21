#include <bits/stdc++.h>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(NULL) {
    }
};

class Solution {
   public:
    ListNode* detectCycle(ListNode* head) {
        ListNode *n1 = head, *n2 = head;
        while (n2 != nullptr) {
            n1 = n1->next;
            n2 = n2->next;
            if (n2 != nullptr) {
                n2 = n2->next;
                if (n1 == n2) {
                    n1 = head;
                    while (n1 != n2) {
                        n1 = n1->next;
                        n2 = n2->next;
                    }
                    return n1;
                }
            }
        }
        return nullptr;
    }
};
