package problem;

import helper.Helper;

public class App044 {
	public static void main(String[] args) {
		new App044().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		int lowestInt = 0;

		boolean found = false;
		int i = 1;

		while (!found) {
			i++;
			int n = this.pentagonalNumber(i);

			for (int j = i - 1; j > 0; j--) {
				int m = this.pentagonalNumber(j);

				if (this.isPentagonalNumber(n - m) && this.isPentagonalNumber(n + m)) {
					lowestInt = n - m;
					found = true;
					break;
				}
			}
		}

		System.out.println(lowestInt);
		Helper.printCalculationTime();
	}

	private int pentagonalNumber(int n) {
		return n * (3 * n - 1) / 2;
	}

	private boolean isPentagonalNumber(int n) {
		double penTest = (Math.sqrt(1 + 24 * n) + 1.0) / 6.0;
		return penTest == ((int) penTest);
	}
}
