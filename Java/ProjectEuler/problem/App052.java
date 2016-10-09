package problem;

import java.math.BigInteger;

import helper.Helper;

public class App052 {
	int num = 6;

	public static void main(String[] args) {
		new App052().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		boolean found = false;
		BigInteger i = BigInteger.valueOf(123456);

		while (!found) {
			boolean breakout = false;

			i = i.add(BigInteger.ONE);
			String givenString = i.toString();

			numLoop: for (int j = 2; j <= this.num; j++) {
				BigInteger searchInt = i.multiply(BigInteger.valueOf(j));
				String searchStr = searchInt.toString();

				if (searchStr.length() > givenString.length()) {
					breakout = true;
					break;
				}

				for (int k = 0; k < givenString.length(); k++) {
					String helpStr = searchStr.replaceFirst(String.valueOf(givenString.charAt(k)), "");

					if (helpStr.equals(searchStr)) {
						breakout = true;
						break numLoop;
					}

					searchStr = helpStr;
				}
			}

			if (!breakout) {
				break;
			}
		}

		System.out.println(i);
		Helper.printCalculationTime();
	}

}
