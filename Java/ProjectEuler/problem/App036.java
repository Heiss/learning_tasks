package problem;

import java.math.BigInteger;

import helper.Helper;

public class App036 {
	int num = (int) Math.pow(10, 6);

	public static void main(String[] args) {
		new App036().run();
	}

	public void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		BigInteger sum = BigInteger.ZERO;

		for (int i = 1; i <= this.num; i++) {
			if (Helper.isPalindrome(BigInteger.valueOf(i)) && Helper.isPalindrome(Helper.getBinary(i))) {
				sum = sum.add(BigInteger.valueOf(i));
			}
		}

		System.out.println(sum);
		Helper.printCalculationTime();
	}
}
