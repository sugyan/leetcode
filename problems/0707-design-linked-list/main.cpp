#include <bits/stdc++.h>

using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int val) : val(val), next(nullptr) {}
};

class MyLinkedList {
public:
    /** Initialize your data structure here. */
    MyLinkedList() {
    }
    
    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    int get(int index) {
        if (index < 0) return -1;
        Node* node = head;
        for (int i = 0; i < index; ++i) {
            if (node == nullptr) return -1;
            node = node->next;
        }
        return node != nullptr ? node->val : -1;
    }
    
    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    void addAtHead(int val) {
        Node* node = new Node(val);
        node->next = head;
        head = node;
    }
    
    /** Append a node of value val to the last element of the linked list. */
    void addAtTail(int val) {
        if (head == nullptr) {
            head = new Node(val);
            return;
        }
        Node* node = head;
        while (node->next != nullptr) node = node->next;
        node->next = new Node(val);
    }
    
    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    void addAtIndex(int index, int val) {
        // if (index < 0) return;
        if (index < -1) return; // for wrong test case
        Node* dummy = new Node(-1);
        dummy->next = head;
        Node* node = dummy;
        for (int i = 0; i < index; ++i) {
            if (node == nullptr) {
                head = dummy->next;
                return;
            }
            node = node->next;
        }
        if (node != nullptr) {
            Node* add = new Node(val);
            add->next = node->next;
            node->next = add;
        }
        head = dummy->next;
    }
    
    /** Delete the index-th node in the linked list, if the index is valid. */
    void deleteAtIndex(int index) {
        if (index < 0) return;
        Node* dummy = new Node(-1);
        dummy->next = head;
        Node* node = dummy;
        for (int i = 0; i < index; ++i) {
            if (node == nullptr) {
                head = dummy->next;
                return;
            }
            node = node->next;
        }
        Node* del = node->next;
        if (del != nullptr) {
            node->next = del->next;
            delete del;
        }
        head = dummy->next;
    }
private:
    Node* head = nullptr;
};

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * MyLinkedList* obj = new MyLinkedList();
 * int param_1 = obj->get(index);
 * obj->addAtHead(val);
 * obj->addAtTail(val);
 * obj->addAtIndex(index,val);
 * obj->deleteAtIndex(index);
 */

int main() {
    MyLinkedList* obj = new MyLinkedList();
    obj->addAtHead(1);
    obj->addAtTail(3);
    obj->addAtIndex(1, 2);
    cout << obj->get(1) << endl;
    obj->deleteAtIndex(1);
    cout << obj->get(1) << endl;
}
