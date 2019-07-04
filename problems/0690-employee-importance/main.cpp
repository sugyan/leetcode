#include <bits/stdc++.h>

using namespace std;

class Employee {
public:
    // It's the unique ID of each node.
    // unique id of this employee
    int id;
    // the importance value of this employee
    int importance;
    // the id of direct subordinates
    vector<int> subordinates;

    Employee(int _id, int _importance, vector<int> _subordinates) {
        id = _id;
        importance = _importance;
        subordinates = _subordinates;
    }
};

class Solution {
public:
    int getImportance(vector<Employee*> employees, int id) {
        unordered_map<int, Employee*> um;
        for (Employee* employee : employees) {
            um[employee->id] = employee;
        }
        return getTotalImportance(um, id);
    }
private:
    int getTotalImportance(unordered_map<int, Employee*>& um, int id) {
        int answer = um[id]->importance;
        for (int subordinate : um[id]->subordinates) {
            answer += getTotalImportance(um, subordinate);
        }
        return answer;
    }
};

int main() {
    vector<Employee*> employees {
        new Employee(1, 5, { 2, 3 }),
        new Employee(2, 3, {}),
        new Employee(3, 3, {}),
    };
    int ret = Solution().getImportance(employees, 1);
    cout << ret << endl;
}
