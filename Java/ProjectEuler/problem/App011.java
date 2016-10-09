package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

import helper.Helper;

public class App011 {
	static File f = new File("./src/ressources/prob11.txt");
	private int num = 20;
	long[][] grid;

	public static void main(String[] args) {
		new App011().run();
	}

	private void run() {
		try (BufferedReader reader = Files.newBufferedReader(f.toPath())) {
			String line = null;
			int i = 0;

			this.grid = new long[this.num][this.num];
			while ((line = reader.readLine()) != null) {
				String[] split = line.split(" ");

				int j = 0;

				for (String string : split) {
					this.grid[i][j++] = Long.valueOf(string);
				}

				i++;
			}
		} catch (IOException x) {
			System.err.format("IOException: %s%n", x);
		}

		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();
		long highestInt = 0;

		for (int i = 0; i < this.num; i++) {
			for (int j = 0; j < this.num; j++) {
				long d = this.down(i, j);
				long r = this.right(i, j);
				long ld = this.leftDiagonally(i, j);
				long rd = this.rightDiagonally(i, j);

				if (d > highestInt) {
					highestInt = d;
				}

				if (r > highestInt) {
					highestInt = r;
				}

				if (ld > highestInt) {
					highestInt = ld;
				}

				if (rd > highestInt) {
					highestInt = rd;
				}
			}
		}

		System.out.println(highestInt);

		Helper.printCalculationTime();
	}

	private long down(int row, int cell) {
		if (row + 3 >= this.num) {
			return -1;
		}

		return this.grid[row][cell] * this.grid[row + 1][cell] * this.grid[row + 2][cell] * this.grid[row + 3][cell];
	}

	private long right(int row, int cell) {
		if (cell + 3 >= this.num) {
			return -1;
		}

		return this.grid[row][cell] * this.grid[row][cell + 1] * this.grid[row][cell + 2] * this.grid[row][cell + 3];
	}

	private long leftDiagonally(int row, int cell) {
		if (row + 3 >= this.num || cell - 3 < 0) {
			return -1;
		}

		return this.grid[row][cell] * this.grid[row + 1][cell - 1] * this.grid[row + 2][cell - 2]
				* this.grid[row + 3][cell - 3];
	}

	private long rightDiagonally(int row, int cell) {
		if (row + 3 >= this.num || cell + 3 >= this.num) {
			return -1;
		}

		return this.grid[row][cell] * this.grid[row + 1][cell + 1] * this.grid[row + 2][cell + 2]
				* this.grid[row + 3][cell + 3];
	}

}
