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
    vector<vector<int>> levelOrder(Node* root) {
        vector<vector<int>> answer;
        if (root == nullptr) return answer;
        queue<pair<int, Node*>> q;
        q.push({ 0, root });
        while (!q.empty()) {
            pair<int, Node*> p = q.front();
            q.pop();
            if (answer.size() < p.first + 1) answer.push_back(vector<int>());
            answer[p.first].push_back(p.second->val);
            for (Node* child : p.second->children) {
                q.push({ p.first + 1, child });
            }
        }
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
    vector<vector<int>> ret = Solution().levelOrder(root);
    for (vector<int> v : ret) {
        for (int n : v) cout << n << " ";
        cout << endl;
    }
}
