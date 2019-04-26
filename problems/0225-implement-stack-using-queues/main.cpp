#include <bits/stdc++.h>

using namespace std;

class MyStack {
    queue<int> q;
public:
    /** Initialize your data structure here. */
    MyStack() {
    }
    
    /** Push element x onto stack. */
    void push(int x) {
        q.push(x);
        for (int i = 0, size = q.size(); i < size - 1; ++i) {
            int v = q.front();
            q.pop();
            q.push(v);
        }
    }
    
    /** Removes the element on top of the stack and returns that element. */
    int pop() {
        int ret = q.front();
        q.pop();
        return ret;
    }
    
    /** Get the top element. */
    int top() {
        return q.front();
    }
    
    /** Returns whether the stack is empty. */
    bool empty() {
        return q.empty();
    }
};

int main() {
    MyStack* stack = new MyStack();
    stack->push(1);
    stack->push(2);
    cout << stack->top() << endl;
    cout << stack->pop() << endl;
    cout << stack->empty() << endl;
}
