package problem;

import java.math.BigInteger;

import helper.Helper;

public class App021 {
	final static int num = 10000;

	public static void main(String[] args) {
		BigInteger sum = BigInteger.ZERO;
		BigInteger zahl1, zahl2;

		for (int i = 0; i < num; i++) {
			BigInteger help = BigInteger.valueOf(i);

			zahl1 = Helper.sumOfDivisors(help);
			zahl2 = Helper.sumOfDivisors(zahl1);

			if (Helper.isPerfectNumber(help)) {
				continue;
			}

			if (help.compareTo(zahl2) == 0) {
				sum = sum.add(help);
			}
		}

		System.out.println(sum);
	}
}
