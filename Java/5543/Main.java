// 상근날드

import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		
		int sangducBurgerPrice = sc.nextInt();
		int jungducBurgerPrice = sc.nextInt();
		int haducBurgerPrice = sc.nextInt();
		int[] burgerLength = {sangducBurgerPrice, jungducBurgerPrice, haducBurgerPrice};
		
		int cokePrice = sc.nextInt();
		int ciderPrice = sc.nextInt();
		int[] drinkLength = {cokePrice, ciderPrice};
		
		int minBurgerPrice = burgerLength[0];
		for (int i = 1; i < burgerLength.length; i++) {
			if (minBurgerPrice >= burgerLength[i]) {
				minBurgerPrice = burgerLength[i];
			}
		}
		
		int minDrinkPrice = drinkLength[0];
		for (int i = 1; i < drinkLength.length; i++) {
			if (minDrinkPrice >= drinkLength[i]) {
				minDrinkPrice = drinkLength[i];
			}
		}
		
		int minSetMenu = minBurgerPrice + minDrinkPrice - 50;
		System.out.println(minSetMenu);
		
	}
}
