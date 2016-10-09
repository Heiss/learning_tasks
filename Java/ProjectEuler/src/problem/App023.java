package problem;

import java.util.ArrayList;

import helper.Helper;

public class App023 {
	int num = 28123;
	ArrayList<Integer> abundantNumbers = new ArrayList<>();
	boolean[] sumAbundants = new boolean[this.num];

	public static void main(String[] args) {
		new App023().run();
	}

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		for (int i = 1; i <= this.num; i++) {
			if (this.isAbundantNumber(i)) {
				this.abundantNumbers.add(i);
			}
		}

		for (Integer frstInt : this.abundantNumbers) {
			for (Integer scndInt : this.abundantNumbers) {
				int c = frstInt + scndInt;
				if (c < this.num) {
					this.sumAbundants[c] = true;
				}
			}
		}

		long sum = 0;
		for (int i = 0; i < this.num; i++) {
			if (!this.sumAbundants[i]) {
				sum += i;
			}
		}

		System.out.println(sum);
		Helper.printCalculationTime();
	}

	private ArrayList<Integer> getDivisors(int number) {
		ArrayList<Integer> res = new ArrayList<>();

		for (int i = 1; i < number; i++) {
			if (number % i == 0) {
				res.add(i);
			}
		}

		return res;
	}

	private boolean isAbundantNumber(int number) {
		ArrayList<Integer> divisors = this.getDivisors(number);

		long sum = 0;
		for (Integer integer : divisors) {
			sum += integer;
		}

		return (sum > number) ? true : false;
	}
}
