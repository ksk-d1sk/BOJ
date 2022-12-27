// 나이순 정렬

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class Main {
	public static void main(String[] args) {
		BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		int n = Integer.parseInt(br.readLine());
		Member[] member = new Member[n];

		for (int i = 0; i < n; i++) {
			StringTokenizer st = new StringTokenizer(br.readLine());
			int age = Integer.parseInt(st.nextToken());
			String name = st.nextToken();
			member[i] = new Member(age, name);
		}

		Sort.mergeSort(member);
		showArray(member);
	}
	
	private static void showArray(Member[] member) {
		StringBuffer sb = new StringBuffer();
		for (Member data : member) {
			sb.append(data.getAge()).append(' ').append(data.getName()).append('\n');
		}
		System.out.println(sb.toString());
	}
}

class Sort {
	public static void mergeSort(Member[] member) {
		Member[] temp = new Member[member.length];
		mergeSort(member, temp, 0, member.length - 1);
	}

	private static void mergeSort(Member[] member, Member[] temp, int start, int end) {
		if (start < end) {
			int mid = (start + end) / 2;
			mergeSort(member, temp, start, mid);
			mergeSort(member, temp, mid + 1, end);
			merge(member, temp, start, mid, end);
		}
	}

	private static void merge(Member[] member, Member[] temp, int start, int mid, int end) {
		for (int i = start; i <= end; i++) {
			temp[i] = member[i];
		}

		int part1 = start;
		int part2 = mid + 1;
		int index = start;

		while (part1 <= mid && part2 <= end) {
			if (temp[part1].getAge() <= temp[part2].getAge()) {
				member[index] = temp[part1];
				part1++;
			} else {
				member[index] = temp[part2];
				part2++;
			}
			index++;
		}

		for (int i = 0; i <= mid - part1; i++) {
			member[index + i] = temp[part1 + i];  
		}
	}
}

class Member {
	private int age;
	private String name;

	public Member(int age, String name) {
		this.age = age;
		this.name = name;
	}

	public int getAge() {
		return age;
	}

	public String getName() {
		return name;
	}
}
