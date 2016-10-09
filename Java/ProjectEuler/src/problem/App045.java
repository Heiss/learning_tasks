package problem;

import helper.Helper;

public class App045 {
	public static void main(String[] args) {
		new App045().run();
	}

	private void run() {
		this.firstAttempt();
		this.secondAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		boolean found = false;
		long i = 0, result = 0;
		int count = 0;

		while (!found) {
			i++;

			if (this.isTriangularNumber(i) && this.isPentagonalNumber(i) && this.isHexagonalNumber(i)) {
				result = i;
				count++;
			}

			if (count == 3) {
				found = true;
			}
		}

		System.out.println(result);
		Helper.printCalculationTime();
	}

	private void secondAttempt() {
		Helper.timerStart();

		long result = 0;
		int i = 143;

		while (true) {
			i++;
			result = i * (2 * i - 1);

			if (this.isPentagonalNumber(result)) {
				break;
			}
		}

		System.out.println(result);
		Helper.printCalculationTime();
	}

	private boolean isPentagonalNumber(long result) {
		double test = (Math.sqrt(24 * result + 1) + 1.0) / 6.0;
		return test == ((int) test);
	}

	private boolean isTriangularNumber(long n) {
		double test = (Math.sqrt(8 * n + 1) - 1.0) / 2.0;
		return test == ((int) test);
	}

	private boolean isHexagonalNumber(long n) {
		double test = (Math.sqrt(8 * n + 1) + 1.0) / 4.0;
		return test == ((int) test);
	}
}
