// 큐

#include <iostream>

int dat[10001];
int tail = 0;
int head = 0;

void push(int);
int pop();
int size();
bool empty();
int front();
int back();

int main() {
	int n;
	std::cin >> n;

	while (n--) {
		std::ios::sync_with_stdio(0);
		std::cin.tie(0);

		std::string cmd;
		std::cin >> cmd;

		if (cmd == "push") {
			int x;
			std::cin >> x;
			push(x);
		}
		else if (cmd == "pop")
			std::cout << pop() << '\n';
		else if (cmd == "size")
			std::cout << size() << '\n';
		else if (cmd == "empty")
			std::cout << empty() << '\n';
		else if (cmd == "front")
			std::cout << front() << '\n';
		else if (cmd == "back")
			std::cout << back() << '\n';
	}

	return 0;
}

void push(int x) {
	dat[tail++] = x;
}

int pop() {
	if (empty())
		return -1;
	else
		return dat[head++];
}

int size() {
	return tail - head;
}

bool empty() {
	if (size() > 0) return 0;
	else return 1;
}

int front() {
	if (empty())
		return -1;
	else
		return dat[head];
}

int back() {
	if (empty())
		return -1;
	else
		return dat[tail-1];
}
