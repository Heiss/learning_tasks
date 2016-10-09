package problem;

import helper.Helper;

public class App015 {
	final static int num = 20;

	public static void main(String[] args) {
		new App015().run();
	}

	private void run() {
		this.firstAttempt();
		this.secondAttempt();
	}

	private void secondAttempt() {
		Helper.timerStart();

		System.out.println(Helper.getBinomialCoefficient(num * 2, num));

		Helper.printCalculationTime();
	}

	// TOOOOOO SLOW :D
	private void firstAttempt() {
		Helper.timerStart();

		System.out.println(this.countPaths(0, 0, 0));

		Helper.printCalculationTime();
	}

	private long countPaths(int index, int a, int b) {
		// if (nGrid.length == 0 || nGrid[0].length == 0) {
		// return 1;
		// }
		//
		// int bottomCell = 0, rightCell = 0;
		//
		// if (index + 1 - num < 0) {
		// int a = nGrid.length - 1, b = nGrid[0].length;
		//
		// rightCell = this.countPaths(index - num, new int[a][b]);
		// }
		//
		// if ((index + num) / num < num) {
		// int a = nGrid.length, b = nGrid[0].length - 1;
		//
		// bottomCell = this.countPaths(index - num, new int[a][b]);
		// }
		//
		// return bottomCell + rightCell;

		long[][] grid = new long[num + 1][num + 1];

		// Initialise the grid with boundary conditions
		for (int i = 0; i < num; i++) {
			grid[i][num] = 1;
			grid[num][i] = 1;
		}

		for (int i = num - 1; i >= 0; i--) {
			for (int j = num - 1; j >= 0; j--) {
				grid[i][j] = grid[i + 1][j] + grid[i][j + 1];
			}
		}

		return grid[0][0];
	}
}
