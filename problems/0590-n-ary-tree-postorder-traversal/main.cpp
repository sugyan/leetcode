#include <bits/stdc++.h>

using namespace std;

class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
public:
    vector<int> postorder(Node* root) {
        vector<int> answer;
        stack<Node*> s;
        if (root != nullptr) s.push(root);
        while (!s.empty()) {
            Node* node = s.top();
            s.pop();
            answer.push_back(node->val);
            for (auto it = node->children.begin(); it != node->children.end(); ++it) {
                s.push(*it);
            }
        }
        reverse(answer.begin(), answer.end());
        return answer;
    }
};

int main() {
    Node* root = new Node(1, vector<Node*> {
        new Node(3, vector<Node*> {
            new Node(5, vector<Node*> {}),
            new Node(6, vector<Node*> {}),
        }),
        new Node(2, vector<Node*> {}),
        new Node(4, vector<Node*> {}),
    });
    vector<int> ret = Solution().postorder(root);
    for (int out : ret) {
        cout << out << endl;
    }
}
