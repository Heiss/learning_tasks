package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.Charset;
import java.nio.file.Files;

import helper.Helper;

public class App013 {
	static File f = new File("./src/ressources/prob13.txt");

	public static void main(String[] args) {
		Helper.timerStart();
		BigInteger res = BigInteger.ZERO;

		Charset charset = Charset.forName("UTF-8");
		try (BufferedReader reader = Files.newBufferedReader(f.toPath(), charset)) {
			String line = null;
			while ((line = reader.readLine()) != null) {
				res = res.add(new BigInteger(line));
			}
		} catch (IOException x) {
			System.err.format("IOException: %s%n", x);
		}

		System.out.println(res.toString().substring(0, 10));

		Helper.printCalculationTime();
	}
}
