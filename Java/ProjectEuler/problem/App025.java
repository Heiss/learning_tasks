package problem;

import java.math.BigInteger;

import helper.Helper;

public class App025 {
	private static int num = 1000;
	private static BigInteger zahl1 = BigInteger.ONE, zahl2 = BigInteger.ONE;

	public static void main(String[] args) {
		BigInteger dig = BigInteger.ZERO;
		BigInteger i = BigInteger.valueOf(2);
		BigInteger fib = BigInteger.ZERO;

		Helper.timerStart();

		while (dig.compareTo(BigInteger.valueOf(num)) < 0) {
			fib = zahl2.add(zahl1);
			zahl1 = zahl2;
			zahl2 = fib;

			// fib = Helper.fib(i); // first Attempt
			i = i.add(BigInteger.ONE);

			dig = Helper.countDigits(fib);
		}

		System.out.println("F_" + i + " = " + fib + " #" + dig);
		Helper.printCalculationTime();
	}
}
