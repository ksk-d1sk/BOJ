// 별 찍기 - 10

#include <iostream>

using namespace std;

bool blankCheck(int n, int x, int y) {
	if (n == 1) {
		return true;
	}

	if (x / (n / 3) % 3 == 1 && y / (n / 3) % 3 == 1) {
		return false;
	}
	
	return blankCheck(n/3, x, y);
}

int main() {
	ios::sync_with_stdio(0);

	int n;
	cin >> n;

	for (int x = 0; x < n; x++) {
		for (int y = 0; y < n; y++) {
			cout << (blankCheck(n, x, y) ? '*' : ' ');
		}
		cout << '\n';
	}

	return 0;
}
