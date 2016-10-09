package problem;

import java.util.ArrayList;

import helper.Helper;

public class App047 {
	int num = 4;
	boolean found = false, isDistinct = false;

	public static void main(String[] args) {
		new App047().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		long i = 1L;

		ArrayList<Long> factors = new ArrayList<>(), resultList = new ArrayList<>();
		while (!this.found) {
			i++;

			factors = Helper.findPrimeFactors(i);
			long countFactors = Helper.countDistinctFactors(factors);

			if (countFactors == this.num) {
				this.isDistinct = true;
			} else {
				this.isDistinct = false;
			}

			if (this.isDistinct) {
				resultList.add(i);
			} else {
				resultList = new ArrayList<>();
			}

			if (resultList.size() == this.num) {
				this.found = true;
			}
		}

		System.out.println(resultList);

		Helper.printCalculationTime();
	}

}
