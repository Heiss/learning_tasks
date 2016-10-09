package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Files;
import java.util.ArrayList;

import helper.Helper;

public class App018 {
	static File f = new File("./src/ressources/prob18.txt");

	public static void main(String[] args) {
		ArrayList<BigInteger> list = new ArrayList<>();

		try (BufferedReader reader = Files.newBufferedReader(f.toPath())) {
			String line = null;
			while ((line = reader.readLine()) != null) {
				String[] split = line.split(" ");

				for (String string : split) {
					list.add(new BigInteger(string));
				}
			}
		} catch (IOException x) {
			System.err.format("IOException: %s%n", x);
		}

		Helper.timerStart();
		System.out.println(sumList(list));
		Helper.printCalculationTime();
	}

	private static BigInteger sumList(ArrayList<BigInteger> list) {
		return getHighestPathSum(list, 0, 1);
	}

	private static BigInteger getHighestPathSum(ArrayList<BigInteger> list, int i, int cRow) {
		if (i >= list.size()) {
			return BigInteger.ZERO;
		}

		BigInteger num = list.get(i);

		i = i + cRow;

		cRow++;
		BigInteger num1 = num.add(getHighestPathSum(list, i, cRow));
		BigInteger num2 = num.add(getHighestPathSum(list, ++i, cRow));

		return num1.max(num2);
	}
}
