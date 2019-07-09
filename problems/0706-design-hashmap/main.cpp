#include <bits/stdc++.h>

using namespace std;

class MyHashMap {
public:
    /** Initialize your data structure here. */
    MyHashMap() {
        v = vector<list<pair<int, int>>>(hashsize);
    }
    
    /** value will always be non-negative. */
    void put(int key, int value) {
        list<pair<int, int>>& l = v[key % hashsize];
        bool exist = false;
        for (list<pair<int, int>>::iterator it = l.begin(); it != l.end(); ++it) {
            if (it->first == key) {
                exist = true;
                it->second = value;
                break;
            }
        }
        if (!exist) l.push_back({ key, value });
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    int get(int key) {
        list<pair<int, int>>& l = v[key % hashsize];
        for (list<pair<int, int>>::iterator it = l.begin(); it != l.end(); ++it) {
            if (it->first == key) {
                return it->second;
            }
        }
        return -1;
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    void remove(int key) {
        list<pair<int, int>>& l = v[key % hashsize];
        for (list<pair<int, int>>::iterator it = l.begin(); it != l.end(); ++it) {
            if (it->first == key) {
                l.erase(it);
                return;
            }
        }
    }
private:
    int hashsize = 256;
    vector<list<pair<int, int>>> v;
};

/**
 * Your MyHashMap object will be instantiated and called as such:
 * MyHashMap* obj = new MyHashMap();
 * obj->put(key,value);
 * int param_2 = obj->get(key);
 * obj->remove(key);
 */

int main() {
    MyHashMap* obj = new MyHashMap();
    obj->put(1, 1);
    obj->put(2, 2);
    cout << obj->get(1) << endl;
    cout << obj->get(3) << endl;
    obj->put(2, 1);
    cout << obj->get(2) << endl;
    obj->remove(2);
    cout << obj->get(2) << endl;
}
