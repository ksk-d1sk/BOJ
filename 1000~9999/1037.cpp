#include <stdio.h>
#include <stdlib.h>

void sort(int arr[], int count)
{
	for (int i = 0; i < count; i++)
	{
		for (int j = 0; j < count - 1; j++)
		{
			if (arr[j] > arr[j + 1])
			{
				int temp = arr[j];
				arr[j] = arr[j + 1];
				arr[j + 1] = temp;
			}
		}
	}
}

int main(void)
{
	int number;
	scanf("%d", &number);

	int* yaksu;
	yaksu = (int*)malloc(sizeof(int) * number);

	for (int i = 0; i < number; i++)
	{
		scanf("%d", &yaksu[i]);
	}

	sort(yaksu, number);

	printf("%d", yaksu[0] * yaksu[number - 1]);

	free(yaksu);

	return 0;
}