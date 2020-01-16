#include <bits/stdc++.h>

using namespace std;

class Node {
   public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {
    }

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {
    }

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {
    }
};

class Solution {
   public:
    Node* connect(Node* root) {
        if (root != nullptr) {
            if (root->left != nullptr) {
                if (root->right != nullptr) {
                    root->left->next = root->right;
                } else {
                    root->left->next = next(root->next);
                }
            }
            if (root->right != nullptr) {
                root->right->next = next(root->next);
            }
            connect(root->right);
            connect(root->left);
        }
        return root;
    }

   private:
    Node* next(Node* node) {
        if (node == nullptr) return node;
        if (node->left != nullptr) return node->left;
        if (node->right != nullptr) return node->right;
        return next(node->next);
    }
};
