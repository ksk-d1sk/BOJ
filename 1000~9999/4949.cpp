// 균형잡힌 세상

#include <iostream>
#include <string>
#include <stack>

using namespace std;

bool balance(string str);

int main() {
	ios::sync_with_stdio(0);
	cin.tie(0);

	while (true) {
		string str;
		getline(cin, str);

		if (str == ".")
			break;

		cout << (balance(str) ? "yes" : "no") << '\n';
	}

	return 0;
}

bool balance(string str) {
	bool result = true;

	stack<char> st;

	for (char i : str) {
		if (i == '(' || i == '[')
			st.push(i);
			
		if (i == ')') {
			if (st.empty() || st.top() != '(') {
				result = false;
				break;
			}
			st.pop();
		}

		if (i == ']') {
			if (st.empty() || st.top() != '[') {
				result = false;
				break;
			}
			st.pop();
		}
	}

	if (!st.empty())
		result = false;

	return result;
}
