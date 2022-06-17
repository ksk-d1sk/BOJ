#include <stdio.h>

int main(void)
{
	int exerciseTime, minPulse, maxPulse, exercise, rest;
	scanf("%d %d %d %d %d", &exerciseTime, &minPulse, &maxPulse, &exercise, &rest);

	int currentPulse = minPulse;
	int elapsedTime = 0;
	
	if (minPulse + exercise > maxPulse)
	{
		elapsedTime = -1;
		exerciseTime = 0;
	}
	
	while (exerciseTime > 0)
	{
		if (currentPulse + exercise <= maxPulse)
		{
			currentPulse += exercise;
			exerciseTime--;
			elapsedTime++;
		}
		else
		{
			currentPulse -= rest;
			
			if (currentPulse < minPulse)
			{
				currentPulse = minPulse;
			}
			
			elapsedTime++;
		}
	}
	
	printf("%d\n", elapsedTime);
	
	return 0;
}
