package problem;

import java.math.BigInteger;

import helper.Helper;

public class App053 {
	int num = 100;

	public static void main(String[] args) {
		new App053().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		long count = 0;
		for (int i = 1; i <= this.num; i++) {
			for (int j = 1; j <= i; j++) {
				BigInteger help = Helper.getBinomialCoefficient(i, j);
				if (help.compareTo(BigInteger.valueOf((long) Math.pow(10, 6))) > 0) {
					count++;
				}
			}
		}

		System.out.println(count);
		Helper.printCalculationTime();
	}

}
