// 스택

#include <iostream>

int dat[10000];
int pos = 0;

void push(int x);
int pop();
int size();
bool empty();
int top();

int main() {
	std::ios::sync_with_stdio(0);
	std::cin.tie(0);

	int n;
	std::cin >> n;

	while (n--) {
		std::string cmd;
		std::cin >> cmd;

		if (cmd == "push") {
			int x;
			std::cin >> x;
			push(x);
		}
		else if (cmd == "pop") std::cout << pop() << '\n';
		else if (cmd == "size") std::cout << size() << '\n';
		else if (cmd == "empty") std::cout << empty() << '\n';
		else if (cmd == "top") std::cout << top() << '\n';
	}	

	return 0;
}

void push(int x) {
	dat[pos++] = x;
}

int pop() {
	if (empty()) return -1;
	else return dat[--pos];
}

int size() {
	return pos;
}

bool empty() {
	if (size()) return 0;
	else return 1;
}

int top() {
	if (empty()) return -1;
	else return dat[pos - 1];
}