package problem;

import java.math.BigInteger;

import helper.Helper;

public class App056 {
	int num = 100;

	public static void main(String[] args) {
		new App056().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		long highestInt = 0;
		for (int i = 1; i < this.num; i++) {
			for (int j = 0; j < this.num; j++) {
				long help = Helper.getSumOfDigits(BigInteger.valueOf(i).pow(j));
				if (help > highestInt) {
					highestInt = help;
				}
			}
		}

		System.out.println(highestInt);
		Helper.printCalculationTime();
	}

}
