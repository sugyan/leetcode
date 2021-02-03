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
    bool hasCycle(ListNode *head) {
        ListNode *node1 = head;
        ListNode *node2 = head;
        while (node1 != nullptr) {
            if (node2 != nullptr) node2 = node2->next;
            if (node2 != nullptr) node2 = node2->next;
            if (node1 == node2) return true;
            node1 = node1->next;
        }
        return false;
    }
};

int main() {
    return 1;
}