// 쇠막대기

#include <iostream>
#include <stack>

using namespace std;

int main() {
	string str;
	cin >> str;

	stack<char> st;
	int count = 0;
	bool isHigh = true;

	for (char i : str) {
		if (i == '(') {
			st.push(i);
			isHigh = true;
		}
		else if (i == ')') {
			st.pop();
			if (isHigh) {
				count += st.size();
				isHigh = false;
			}
			else {
				count += 1;
			}
		}
	}

	cout << count << '\n';

	return 0;
}
