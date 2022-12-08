// 진짜 공간

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        byte n = Byte.parseByte(br.readLine());
        int[] arr = new int[n];
        StringTokenizer st = new StringTokenizer(br.readLine());
        for (int i = 0; i < n; i++) {
            arr[i] = (Integer.parseInt(st.nextToken()));
        }
        int cluster = Integer.parseInt(br.readLine());
        long result = Main.solution(arr, cluster);
        System.out.println(result);
    }

    static long solution(int[] arr, int cluster) {
        long sum = 0;
        for (int i : arr) {
            sum += i / cluster * cluster;
            if (i % cluster != 0) { sum += cluster; }
        }
        return sum;
    }
}
