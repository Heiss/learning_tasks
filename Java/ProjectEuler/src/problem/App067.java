package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Files;
import java.util.ArrayList;

import helper.Helper;

public class App067 {
	static File f = new File("./src/ressources/prob67.txt");

	public static void main(String[] args) {
		Helper.timerStart();
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

		int cRow = 1;
		ArrayList<ArrayList<BigInteger>> triangle = new ArrayList<>();

		while (list.size() > 0) {
			ArrayList<BigInteger> nList = new ArrayList<>();

			for (int i = 0; i < cRow; i++) {
				nList.add(list.remove(0));
			}

			triangle.add(nList);
			cRow++;
		}

		for (int i = triangle.size() - 2; i >= 0; i--) {
			for (int j = 0; j < triangle.get(i).size(); j++) {
				ArrayList<BigInteger> currentRow = triangle.get(i);
				ArrayList<BigInteger> nextRow = triangle.get(i + 1);
				BigInteger num = currentRow.get(j).add(nextRow.get(j).max(nextRow.get(j + 1)));

				currentRow.set(j, num);
			}
		}

		System.out.println(triangle.get(0).get(0));

		Helper.printCalculationTime();
	}
}
