// 별 찍기 - 13

#include <iostream>

using namespace std;

void recursive(int n, int i);
void printStar(int n);

int main() {
  int n;
  cin >> n;
  recursive(n, 1);
}

void recursive(int n, int i) {
  if(n <= i) {
    printStar(i);
    return;
  }

  printStar(i);
  cout << '\n';
  recursive(n, i + 1);
  cout << '\n';
  printStar(i);
}

void printStar(int n) {
  for (int i = 0; i < n; i++)
    cout << '*';
}
