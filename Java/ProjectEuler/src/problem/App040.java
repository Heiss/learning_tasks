package problem;

import java.math.BigInteger;
import java.util.ArrayList;

import helper.Helper;

public class App040 {
	int[] numbers = { 1, 10, 100, 1000, 10000, 100000, 1000000 };

	public static void main(String[] args) {
		new App040().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		boolean exit = false;
		int ret = 1, zahl = 0;
		BigInteger bnumbers, count = BigInteger.ZERO, i = BigInteger.ZERO, diff;

		ArrayList<String> listZahl = new ArrayList<>();

		for (int k : this.numbers) {
			exit = false;
			bnumbers = BigInteger.valueOf(k);

			while (exit == false) {
				if (count.compareTo(bnumbers) >= 0) {
					exit = true;
				} else {
					i = i.add(BigInteger.ONE);
					count = count.add(Helper.countDigits(i));
				}
			}

			diff = count.subtract(bnumbers);
			diff = BigInteger.valueOf(i.toString().length()).subtract(diff).subtract(BigInteger.ONE);

			zahl = Character.getNumericValue(i.toString().charAt(diff.intValue()));

			ret *= zahl;
			listZahl.add(String.valueOf(zahl));
		}

		String str = String.join("*", listZahl);

		System.out.println(str + " = " + ret);
		Helper.printCalculationTime();
	}
}
