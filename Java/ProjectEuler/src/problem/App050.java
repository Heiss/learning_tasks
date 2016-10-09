package problem;

import java.util.ArrayList;

import helper.Helper;

public class App050 {

	int num = (int) Math.pow(10, 6);

	public static void main(String[] args) {
		new App050().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		long highestRes = -1, res;
		long maxTerms = 0, terms = 0;

		// its implicit that the numbers are under 5000, otherwise the primes
		// are too big to make the longest sum of terms
		ArrayList<Long> primeList = Helper.getSimplePrimeList(this.num / 20);

		while (primeList.size() > 0) {
			res = 0;
			terms = 0;

			for (Long current : primeList) {
				res += current;

				if (res > this.num) {
					break;
				}

				terms++;
				if (terms > maxTerms && highestRes < res && Helper.isPrime(res)) {
					highestRes = res;
					maxTerms = terms;
				}
			}

			primeList.remove(0);
		}

		System.out.println(maxTerms + ", " + highestRes);

		Helper.printCalculationTime();
	}

}
