package problem;

import helper.Helper;

public class App010 {
	private static long number = (long) (2 * Math.pow(10, 6));

	public static void main(String[] args) {
		long res = 5;
		int j = 0;

		for (int i = 0; i < number; i++) {
			if (i % 2 == 0) {
				continue;
			}
			if (i % 3 == 0) {
				continue;
			}

			if (Helper.isPrime(i)) {
				res += i;
				if (j % 100 == 0) {
					System.out.println(i);
				}
				j++;
			}
		}

		System.out.println("# " + res);
	}

}
