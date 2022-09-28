// 소수

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Main {
	
	int sum = 0;
	
	public static void main(String[] args) {
		List<Integer> array = new ArrayList<>();
		
		Scanner sc = new Scanner(System.in);
		int m = sc.nextInt();
		int n = sc.nextInt();
		sc.close();
		
		for (int i = 2; i <= n; i++) {
			boolean check = true;
			for (int j = 0; j < array.size(); j++) {
				if (i % array.get(j) == 0) {
					check = false;
					break;
				}
			}
			if (check) {
				array.add(i);
			}
		}

		Main main = new Main();
		(array = array.stream()
				.filter(i -> m <= i && i <= n)
				.collect(Collectors.toList()))
				.forEach(i -> main.sum += i);
		
		if (array.size() > 0) {			
			System.out.printf("%d\n%d\n", main.sum, array.get(0));
		} else {
			System.out.println(-1);
		}
	}
}
