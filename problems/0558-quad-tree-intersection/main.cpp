#include <bits/stdc++.h>

using namespace std;

class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;

    Node() {}

    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};

class Solution {
public:
    Node* intersect(Node* quadTree1, Node* quadTree2) {
        if (quadTree1->isLeaf) {
            if (quadTree1->val) return quadTree1;
            return quadTree2;
        }
        if (quadTree2->isLeaf) {
            if (quadTree2->val) return quadTree2;
            return quadTree1;
        }
        Node* tl = intersect(quadTree1->topLeft,     quadTree2->topLeft);
        Node* tr = intersect(quadTree1->topRight,    quadTree2->topRight);
        Node* bl = intersect(quadTree1->bottomLeft,  quadTree2->bottomLeft);
        Node* br = intersect(quadTree1->bottomRight, quadTree2->bottomRight);
        if (tl->val && tr->val && bl->val && br->val) {
            return new Node(true, true, nullptr, nullptr, nullptr, nullptr);
        }
        return new Node(false, false, tl, tr, bl, br);
    }
};

int main() {
    Node* A = new Node(false, false,
        new Node(true,  true, nullptr, nullptr, nullptr, nullptr),
        new Node(true,  true, nullptr, nullptr, nullptr, nullptr),
        new Node(false, true, nullptr, nullptr, nullptr, nullptr),
        new Node(false, true, nullptr, nullptr, nullptr, nullptr));
    Node* B = new Node(false, false,
        new Node(true,  true, nullptr, nullptr, nullptr, nullptr),
        new Node(false, false,
            new Node(false, true, nullptr, nullptr, nullptr, nullptr),
            new Node(false, true, nullptr, nullptr, nullptr, nullptr),
            new Node(true,  true, nullptr, nullptr, nullptr, nullptr),
            new Node(true,  true, nullptr, nullptr, nullptr, nullptr)),
        new Node(true,  true, nullptr, nullptr, nullptr, nullptr),
        new Node(false, true, nullptr, nullptr, nullptr, nullptr));
    Node* ret = Solution().intersect(A, B);
    cout << ret->topLeft->val     << endl;
    cout << ret->topRight->val    << endl;
    cout << ret->bottomLeft->val  << endl;
    cout << ret->bottomRight->val << endl;
}
