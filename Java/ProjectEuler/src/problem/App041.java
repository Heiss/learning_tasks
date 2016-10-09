package problem;

import java.math.BigInteger;
import java.util.ArrayList;

import helper.Helper;

public class App041 {
	private ArrayList<BigInteger> permList = new ArrayList<>();

	public static void main(String[] args) {
		new App041().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();
		BigInteger highestInt = BigInteger.ZERO;

		int numbers = 0;

		for (int j = 1; j <= 9; j++) {
			this.permList.clear();

			numbers = j;
			String ret = "";

			for (int i = 1; i <= numbers; i++) {
				ret += String.valueOf(i);
			}

			this.permList = Helper.getPermutationList(ret);

			for (BigInteger zahl : this.permList) {
				if (Helper.isPrime(zahl.intValue()) && highestInt.compareTo(zahl) <= 0) {
					highestInt = zahl;
				}
			}
		}

		System.out.println("highest, pandigital prime with " + numbers + "-digit: " + highestInt);

		Helper.printCalculationTime();
	}
}
