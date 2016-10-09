package problem;

import helper.Helper;

public class App035 {

	public static void main(String[] args) {
		new App035().run();
	}

	public void run() {
		Helper.timerStart();
		int numbers = (int) Math.pow(10, 6);
		int count = 0;

		for (int i = 2; i < numbers; i++) {
			if (!Helper.isPrime(i)) {
				continue;
			}

			if (rotationIsPrime(String.valueOf(i))) {
				count++;
			}
		}

		System.out.println(count);
		Helper.printCalculationTime();
	}

	private static boolean rotationIsPrime(String str) {
		String help = str.substring(0, 1);
		String nStr = str.substring(1);
		nStr += help;

		for (int i = 0; i < str.length(); i++) {
			if (!Helper.isPrime(Long.valueOf(nStr))) {
				return false;
			}

			help = nStr.substring(0, 1);
			nStr = nStr.substring(1);
			nStr += help;
		}

		return true;
	}
}
