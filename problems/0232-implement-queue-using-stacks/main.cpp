#include <bits/stdc++.h>

using namespace std;

class MyQueue {
    stack<int> s;
public:
    /** Initialize your data structure here. */
    MyQueue() {
    }
    
    /** Push element x to the back of queue. */
    void push(int x) {
        stack<int> tmp;
        while (!s.empty()) {
            tmp.push(s.top());
            s.pop();
        }
        s.push(x);
        while (!tmp.empty()) {
            s.push(tmp.top());
            tmp.pop();
        }
    }
    
    /** Removes the element from in front of queue and returns that element. */
    int pop() {
        int ret = s.top();
        s.pop();
        return ret;
    }
    
    /** Get the front element. */
    int peek() {
        return s.top();
    }
    
    /** Returns whether the queue is empty. */
    bool empty() {
        return s.empty();
    }
};

int main() {
    MyQueue* queue = new MyQueue();
    queue->push(1);
    queue->push(2);
    cout << queue->peek() << endl;
    cout << queue->pop() << endl;
    cout << queue->empty() << endl;
}
