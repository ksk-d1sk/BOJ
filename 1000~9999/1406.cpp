// ¿¡µğÅÍ

#include <iostream>
#include <algorithm>

#define MX 600001

char dat[MX];
int pre[MX], nxt[MX];
int unused = 1;

void insert(int addr, char text)
{
	dat[unused] = text;
	pre[unused] = addr;
	nxt[unused] = nxt[addr];
	if (nxt[addr] != -1) pre[nxt[addr]] = unused;
	nxt[addr] = unused;
	unused++;
}

void erase(int addr)
{
	nxt[pre[addr]] = nxt[addr];
	if (nxt[addr] != -1) pre[nxt[addr]] = pre[addr];
}

void traverse()
{
	int cur = nxt[0];
	while (cur != -1)
	{
		std::cout << dat[cur];
		cur = nxt[cur];
	}
}

int main()
{
	std::ios::sync_with_stdio(false);
	std::cin.tie(nullptr);

	std::fill(pre, pre + MX, -1);
	std::fill(nxt, nxt + MX, -1);

	std::string str;
	std::cin >> str;

	int n = str.length();
	int m;
	std::cin >> m;

	for (int i = 0; i < str.length(); i++)
		insert(i, str[i]);

	while (m--)
	{
		char cmd;
		std::cin >> cmd;
		switch (cmd)
		{
		case 'L':
			if (pre[n] != -1) n = pre[n];
			break;
		case 'D':
			if (nxt[n] != -1) n = nxt[n];
			break;
		case 'B':
			if (pre[n] != -1)
			{
				erase(n);
				n = pre[n];
			}
			break;
		case 'P':
			char text;
			std::cin >> text;
			insert(n, text);
			n = nxt[n];
			break;
		}
	}
	traverse();
	return 0;
}