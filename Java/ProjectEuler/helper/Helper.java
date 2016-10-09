package helper;

import java.math.BigInteger;
import java.util.ArrayList;

public final class Helper {
	private static long timer;

	public static void timerStart() {
		timer = System.currentTimeMillis();
		System.out.println("Timer and calculation started.");
	}

	public static long timerDifference() {
		return System.currentTimeMillis() - timer;
	}

	public static void printCalculationTime() {
		System.out.println("Calculation took " + timerDifference() + "ms\n");
	}

	/**
	 * Return an arraylist with all primes between 2 and numb
	 *
	 * @param numb
	 * @return
	 */
	public static ArrayList<Long> getSimplePrimeList(long numb) {
		return getPrimeList(2, numb);
	}

	/**
	 * Return an arraylist with all primes between start and end
	 *
	 * @param start
	 * @param end
	 * @return
	 */
	public static ArrayList<Long> getPrimeList(long start, long end) {
		long count = start;
		ArrayList<Long> primeList = new ArrayList<>();

		while (count < end) {
			if (isPrime(count)) {
				primeList.add(count);
			}
			count++;
		}

		return primeList;
	}

	public static long ggt(long zahl1, long zahl2) {
		while (zahl2 != 0) {
			if (zahl1 > zahl2) {
				zahl1 = zahl1 - zahl2;
			} else {
				zahl2 = zahl2 - zahl1;
			}
		}
		return zahl1;
	}

	/**
	 * factorial of number
	 *
	 * @param num
	 * @return
	 */
	public static BigInteger fac(long num) {
		BigInteger sum = BigInteger.valueOf(1L);

		for (int i = 1; i <= num; i++) {
			sum = sum.multiply(BigInteger.valueOf(i));
		}

		return sum;
	}

	/**
	 * Sums the digits from the given number
	 *
	 * @param bigInteger
	 * @return
	 */
	public static long getSumOfDigits(BigInteger bigInteger) {
		long ret = 0;
		String str = bigInteger.toString();

		for (int i = 0; i < str.length(); i++) {
			ret += Character.getNumericValue(str.charAt(i));
		}

		return ret;
	}

	public static BigInteger sumOfDivisors(BigInteger num) {
		BigInteger sum = BigInteger.ZERO;

		for (BigInteger i = BigInteger.ONE; i.compareTo(num.divide(BigInteger.valueOf(2))) < 1; i = i
				.add(BigInteger.ONE)) {
			if (num.mod(i) == BigInteger.ZERO) {
				sum = sum.add(i);
			}
		}

		return sum;

	}

	public static BigInteger fib(BigInteger zahl) {
		if (zahl.compareTo(BigInteger.ONE) == 0 || zahl.compareTo(BigInteger.valueOf(2)) == 0) {
			return BigInteger.ONE;
		}

		return fib(zahl.subtract(BigInteger.ONE)).add(fib(zahl.subtract(BigInteger.ONE.add(BigInteger.ONE))));
	}

	public static BigInteger countDigits(BigInteger num) {
		return BigInteger.valueOf(num.toString().length());
	}

	/**
	 * Only works if you called the Helper.findPrimeFactors!
	 *
	 * @param factors
	 * @return
	 */
	public static long countDistinctFactors(ArrayList<Long> factors) {
		if (factors.size() == 1) {
			return 1;
		}

		long c = 0L;

		long previous = -1;
		for (Long current : factors) {
			if (previous == -1 || previous != current) {
				c++;
			}

			previous = current;
		}

		return c;
	}

	private static ArrayList<BigInteger> permList;

	public static ArrayList<BigInteger> getPermutationList(String str) {
		permList = new ArrayList<>();
		permutation("", str);

		return permList;
	}

	private static String permutation(String prefix, String str) {
		int n = str.length();

		if (n > 0) {
			for (int i = 0; i < n; i++) {
				permutation(prefix + str.charAt(i), str.substring(0, i) + str.substring(i + 1, n));
			}
		}

		if (n == 0) {
			permList.add(new BigInteger(prefix));
		}

		return prefix;
	}

	public static ArrayList<Long> findPrimeFactors(long n) {
		ArrayList<Long> numbs = new ArrayList<>();
		Long counter = 2L;

		while (counter <= (int) Math.sqrt(n) && n > 1) {
			if (n % counter == 0) {
				n /= counter;
				numbs.add(counter);
			} else {
				counter++;
			}
		}

		if (n != 1) {
			numbs.add(n);
		}

		return numbs;
	}

	public static long findNextPrime(long num) {
		long j = num;
		while (!Helper.isPrime(j)) {
			j++;
		}
		return j;
	}

	public static boolean isPrime(String number) {
		return isPrime(Long.valueOf(number));
	}

	/**
	 * Check whether the given number is a prime or not with euclid and some
	 * tweaks
	 *
	 * @param number
	 * @return
	 */
	public static boolean isPrime(long number) {
		if (number < 2) {
			return false;
		}

		if (number == 2 || number == 3) {
			return true;
		}

		if ((number & 1) == 0 || number % 3 == 0) {
			return false;
		}

		int sqrtN = (int) Math.sqrt(number) + 1;

		for (int i = 6; i <= sqrtN; i += 6) {// loop 6 step
			if (number % (i - 1) == 0 || number % (i + 1) == 0) {
				return false;
			}
		}

		return true;
	}

	/**
	 * Check whether the given number is a prime or not with euclid
	 *
	 * @param prime
	 * @return
	 */
	public static boolean isPrime2(long prime) {
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

	public static boolean isPerfectNumber(BigInteger num) {
		return num.compareTo(sumOfDivisors(num)) == 0;
	}

	public static boolean isPalindrome(BigInteger bigInteger) {
		String str = String.valueOf(bigInteger);

		int i = 0;
		while (i < str.length() / 2) {
			if (str.charAt(i) != str.charAt(str.length() - 1 - i)) {
				return false;
			}
			i++;
		}
		return true;
	}

	public static BigInteger getBinary(int num) {
		String res = "";

		while (num > 0) {
			res = (num % 2) + res;
			num = num / 2;
		}

		return new BigInteger(res);
	}

	public static BigInteger getBinomialCoefficient(int n, int k) {
		return Helper.fac(n).divide(Helper.fac(k).multiply(Helper.fac(n - k)));
	}

	public static void printArray(long[][] grid) {
		for (long[] row : grid) {
			for (int j = 0; j < grid.length; j++) {
				System.out.print(row[j] + " ");
			}
			System.out.print("\n");
		}
	}

}
