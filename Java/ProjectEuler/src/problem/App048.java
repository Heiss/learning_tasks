package problem;

import java.math.BigInteger;

import helper.Helper;

public class App048 {
	int num = 1000;
	int digits = 10;

	public static void main(String[] args) {
		new App048().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		BigInteger res = BigInteger.ZERO;

		for (int i = 1; i <= this.num; i++) {
			res = res.add(BigInteger.valueOf(i).pow(i));
		}

		String str = res.toString().substring(res.toString().length() - this.digits);
		System.out.println(str);

		Helper.printCalculationTime();
	}

}
