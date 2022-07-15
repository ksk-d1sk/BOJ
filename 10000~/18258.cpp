// 큐 2

#include <iostream>

using namespace std;

void push(int);
int pop();
int size();
bool empty();
int front();
int back();

int dat[2000001];
int tail = 0;
int head = 0;

int main() {
	int n;
	cin >> n;

	while (n--) {
		ios::sync_with_stdio(0);
		cin.tie(0);

		string cmd;
		cin >> cmd;

		if (cmd == "push") {
			int x;
			cin >> x;
			push(x);
		}
		else if (cmd == "pop")
			cout << pop() << '\n';
		else if (cmd == "size")
			cout << size() << '\n';
		else if (cmd == "empty")
			cout << empty() << '\n';
		else if (cmd == "front")
			cout << front() << '\n';
		else if (cmd == "back")
			cout << back() << '\n';
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
	if (size())
        	return 0;
	else
        	return 1;
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
