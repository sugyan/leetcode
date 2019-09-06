#include <bits/stdc++.h>

using namespace std;

class RecentCounter {
public:
    RecentCounter() {
    }
    int ping(int t) {
        q.push(t);
        while (q.front() < t - 3000) q.pop();
        return q.size();
    }
private:
    queue<int> q;
};

int main() {
    RecentCounter* obj = new RecentCounter();
    cout << obj->ping(1) << endl;
    cout << obj->ping(100) << endl;
    cout << obj->ping(3001) << endl;
    cout << obj->ping(3002) << endl;
}
