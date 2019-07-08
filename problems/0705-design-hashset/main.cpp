#include <bits/stdc++.h>

using namespace std;

class MyHashSet {
public:
    /** Initialize your data structure here. */
    MyHashSet() {
        v = vector<bool>(1000001, false);
    }
    
    void add(int key) {
        v[key] = true;
    }
    
    void remove(int key) {
        v[key] = false;
    }
    
    /** Returns true if this set contains the specified element */
    bool contains(int key) {
        return v[key];
    }
private:
    vector<bool> v;
};

/**
 * Your MyHashSet object will be instantiated and called as such:
 * MyHashSet* obj = new MyHashSet();
 * obj->add(key);
 * obj->remove(key);
 * bool param_3 = obj->contains(key);
 */

int main() {
    MyHashSet* obj = new MyHashSet();
    obj->add(1);
    obj->add(2);
    cout << obj->contains(1) << endl;
    cout << obj->contains(3) << endl;
    obj->add(2);
    cout << obj->contains(2) << endl;
    obj->remove(2);
    cout << obj->contains(2) << endl;
}
