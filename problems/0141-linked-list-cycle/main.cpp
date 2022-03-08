#include <bits/stdc++.h>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {
    }
};

class Solution {
   public:
    bool hasCycle(ListNode *head) {
        for (ListNode *slow = head, *fast = head; slow != nullptr;
             slow = slow->next) {
            if (fast != nullptr) fast = fast->next;
            if (fast != nullptr) fast = fast->next;
            if (slow == fast) return true;
        }
        return false;
    }
};
