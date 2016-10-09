package problem;

import java.math.BigInteger;

import helper.Helper;

public class App034 {
	static int numbers = 1000000;

	public static void main(String[] args) {
		new App034().run();
	}

	public void run() {
		Helper.timerStart();
		BigInteger sum = BigInteger.ZERO;

		for (int i = 3; i < numbers; i++) {
			BigInteger bInt = BigInteger.valueOf(i);
			String val = bInt.toString();
			BigInteger vInt = BigInteger.ZERO;

			for (int j = 0; j < val.length(); j++) {
				vInt = vInt.add(Helper.fac(Character.getNumericValue(val.charAt(j))));
			}

			if (vInt.compareTo(bInt) == 0) {
				sum = sum.add(bInt);
			}
		}

		System.out.println(sum);
		Helper.printCalculationTime();
	}
}
