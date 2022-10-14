#include <iostream>
#include <stack>

using namespace std;

bool solution();

int main() {
    int t;
    cin >> t;

    for (int i = 0; i < t; i++) {
        cout << (solution() ? "YES" : "NO") << '\n';
    }

    return 0;
}

bool solution() {
    string str;
    cin >> str;

    stack<char> st;

    bool check = true;

    for (int i = 0; i < str.length(); i++) {
        if (str[i] == '(') {
            st.push(str[i]);
        }
        else {
            if (!st.empty() && st.top() == '(') {
                st.pop();
            }
            else {
                check = false;
                break;
            }
        }
    }

    if (!st.empty()) {
        check = false;
    }

    return check;
}
