package problem;

import java.util.ArrayList;

public class App007 {
	public final static int numbers = 10001;

	public static void main(String[] args) {
		int c = 2, i = 1;
		ArrayList<Integer> primeList = new ArrayList<>();

		while (primeList.size() < numbers) {
			if (isPrime(c)) {
				primeList.add(c);
				System.out.println("#" + i + ": " + c);
				i++;
			}
			c++;
		}

		System.out.println(primeList.get(primeList.size() - 1));
	}

	public static boolean isPrime(long prime) {
		if (prime == 0) {
			return false;
		}
		if (prime == 1) {
			return true;
		}

		long c = 1;
		while (ggt(prime, c) == 1) {
			c++;
		}

		if (c == prime) {
			return true;
		}
		return false;
	}

	private static long ggt(long zahl1, long zahl2) {
		while (zahl2 != 0) {
			if (zahl1 > zahl2) {
				zahl1 = zahl1 - zahl2;
			} else {
				zahl2 = zahl2 - zahl1;
			}
		}
		return zahl1;
	}
}
