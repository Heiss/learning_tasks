package problem;

import java.math.BigInteger;

import helper.Helper;

public class App016 {
	final static long base = 2L;
	final static int exponent = 1000;

	public static void main(String[] args) {
		firstAttempt();
		System.out.println("");
		secondAttempt();
	}

	private static void firstAttempt() {
		BigInteger x = BigInteger.valueOf(base);
		long y = exponent;
		BigInteger pot = BigInteger.ONE;

		while (true) {
			if (y == 0 || y == 1) {
				y--;
				break;
			} else if (y % 2 > 0) {
				pot = pot.multiply(x);
				y--;
			} else {
				x = x.pow(2);
				y /= 2;
			}
		}

		System.out.println("x=" + x + ", y=" + y + ", pot=" + pot);

		BigInteger res = pot.multiply(x);
		System.out.println("Ergebnis: " + res);
		System.out.println("sum of res=" + Helper.getSumOfDigits(res));
	}

	private static void secondAttempt() {
		int result = 0;
		String val = BigInteger.valueOf(base).pow(exponent).toString();

		for (char a : val.toCharArray()) {
			result = result + Character.getNumericValue(a);
		}
		System.out.println("val ==>" + result);
	}

}
