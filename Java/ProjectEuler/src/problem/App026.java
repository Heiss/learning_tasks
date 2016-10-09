package problem;

import helper.Helper;

public class App026 {
	int num = 1000;

	public static void main(String[] args) {
		new App026().run();
	}

	private void run() {
		// this.firstAttempt();
		this.secondAttempt();
	}

	private void secondAttempt() {
		Helper.timerStart();

		int highestInt = 0;

		for (int d = 2; d < 1000; d++) {
			if (highestInt > d) {
				break;
			}

			int[] foundRemainders = new int[d];
			int value = 1;
			int position = 0;

			while (foundRemainders[value] == 0 && value != 0) {
				foundRemainders[value] = position;
				value *= 10;
				value %= d;
				position++;
			}

			if (position - foundRemainders[value] > highestInt) {
				highestInt = position - foundRemainders[value];
			}
		}

		System.out.println(highestInt);

		Helper.printCalculationTime();
	}

	@SuppressWarnings("unused")
	private void firstAttempt() {
		Helper.timerStart();

		int highestInt = 0;
		for (int i = 1; i < this.num; i++) {
			double c = ((double) 1) / i;

			String tmp = String.valueOf(c);

			for (int j = tmp.length() - 1; j >= 0; j--) {
				if (tmp.charAt(j) == 0) {
					tmp = tmp.substring(0, tmp.length() - 1);
				}
			}

			for (int j = tmp.length() - 1; j >= 0; j--) {
				if (j > 3 && tmp.charAt(j) == tmp.charAt(j - 1)) {
					tmp = tmp.substring(0, tmp.length() - 1);
				}
			}

			System.out.println(tmp);

			for (int k = 2; k < tmp.length(); k++) {
				for (int j = k + 1; j < tmp.length(); j++) {
					if (tmp.charAt(k) == tmp.charAt(j) && j > highestInt) {
						highestInt = j - 2;
					}
				}
			}
		}

		System.out.println(highestInt);

		Helper.printCalculationTime();
	}
}
