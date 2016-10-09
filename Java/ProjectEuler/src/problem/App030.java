package problem;

import java.math.BigInteger;

import helper.Helper;

public class App030 {
	static int power = 5;

	public static void main(String[] args) {
		Helper.timerStart();

		BigInteger sum = BigInteger.ZERO;
		BigInteger i = new BigInteger("2");

		while (i.compareTo(BigInteger.valueOf(999999)) < 0) {
			if (sumDigitsWithPower(i).compareTo(i) == 0) {
				sum = sum.add(i);
			}
			i = i.add(BigInteger.ONE);
		}

		System.out.println(sum.toString());
		Helper.printCalculationTime();
	}

	private static BigInteger sumDigitsWithPower(BigInteger j) {
		BigInteger ret = BigInteger.ZERO;
		String str = j.toString();

		for (int i = 0; i < str.length(); i++) {
			ret = ret.add(BigInteger.valueOf(Character.getNumericValue(str.charAt(i))).pow(power));
		}

		return ret;
	}
}
