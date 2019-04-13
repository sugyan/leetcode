#include <bits/stdc++.h>

using namespace std;

class MinStack {
    stack<int> s;
    stack<int> m;

public:
    /** initialize your data structure here. */
    MinStack() {
    }
    
    void push(int x) {
        s.push(x);
        if (m.empty() || x <= m.top()) {
            m.push(x);
        }
    }
    
    void pop() {
        if (s.top() == m.top()) {
            m.pop();
        }
        s.pop();
    }
    
    int top() {
        return s.top();
    }
    
    int getMin() {
        return m.top();
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(x);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */
