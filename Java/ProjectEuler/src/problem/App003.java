package problem;

import java.util.ArrayList;
import java.util.stream.Collectors;

import helper.Helper;

public class App003 {
	public static ArrayList<Long> primeList = new ArrayList<Long>();
	// public static long numb = 13195L;
	public static long numb = 12L;

	public static void main(String[] args) {
		new App003().run();
	}

	private void run() {
		// firstAttempt();
		this.secondAttempt();
	}

	@SuppressWarnings("unused")
	private void firstAttempt() {
		ArrayList<Long> resList = new ArrayList<Long>();
		Long prime;

		this.getPrimeList();
		System.out.println("###### primelist done");

		int i = 0;
		while (numb > 1) {
			prime = primeList.get(i);

			System.out.println(prime + ", " + numb);

			if (numb % prime == 0) {
				resList.add(prime);
				numb /= prime;
			} else {
				++i;
			}
		}

		this.echoPrimes(resList);

		System.out.println("largest Primefactor: " + resList.get(resList.size() - 1));
	}

	private void secondAttempt() {
		System.out.println(Helper.findPrimeFactors(numb));
	}

	private void echoPrimes(ArrayList<Long> getList) {
		String primes = getList.stream().map(Object::toString).collect(Collectors.joining(", "));
		System.out.println(primes);
	}

	private void getPrimeList() {
		long count = 2;
		while (count < Math.sqrt(numb)) {
			if (this.isPrime(count)) {
				primeList.add(count);
			}
			count++;
		}
	}

	public boolean isPrime(long prime) {
		if (prime == 0) {
			return false;
		}
		if (prime == 1) {
			return true;
		}

		long c = 1;
		while (this.ggt(prime, c) == 1) {
			c++;
		}

		if (c == prime) {
			return true;
		}
		return false;
	}

	private long ggt(long zahl1, long zahl2) {
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
