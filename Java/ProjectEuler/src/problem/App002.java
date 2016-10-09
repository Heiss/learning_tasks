package problem;

import java.math.BigInteger;

import helper.Helper;

public class App002 {

	private static long num = (long) (4 * Math.pow(10, 6));

	public static void main(String[] args) {
		BigInteger c = BigInteger.ZERO;
		BigInteger value = BigInteger.ZERO;
		BigInteger res = BigInteger.ZERO;

		while (value.compareTo(BigInteger.valueOf(num)) < 0) {
			c = c.add(BigInteger.ONE);
			value = Helper.fib(c);

			System.out.println(value);

			if (value.mod(BigInteger.valueOf(2)).compareTo(BigInteger.ZERO) == 0) {
				res = res.add(value);
			}
		}

		System.out.println(res);
	}

}
