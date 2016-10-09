package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;

import helper.Helper;

public class App059 {
	int keyLength = 3;
	ArrayList<Character> message = new ArrayList<>();

	public static void main(String[] args) {
		new App059().run();
	}

	private void run() {
		File f = new File("./src/ressources/prob59.txt");
		try (BufferedReader reader = Files.newBufferedReader(f.toPath())) {
			String line = null;
			while ((line = reader.readLine()) != null) {
				String[] split = line.split(",");

				for (String string : split) {
					int tmp = Integer.valueOf(string);
					this.message.add((char) tmp);
				}
			}
		} catch (IOException x) {
			System.err.format("IOException: %s%n", x);
		}

		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		StringBuilder res = new StringBuilder();
		int[] key = new int[this.keyLength];

		for (int i = 0; i < key.length; i++) {
			key[i] = 97;
		}

		bigLoop: while (true) {
			if (key[0] > 122) {
				break;
			}

			for (int j = 97; j <= 122; j++) {
				key[2] = j;

				res = this.translateMessage(key);

				if (res.indexOf(" the ") > -1) {
					break bigLoop;
				}
			}

			if (key[1] >= 122) {
				key[0] += 1;
				key[1] = 97;
			} else {
				key[1] += 1;
			}
		}

		int sum = 0;
		for (char c : res.toString().toCharArray()) {
			sum += c;
		}

		System.out.println(res);
		System.out.println("key: " + this.keyToString(key) + ", sum: " + sum);
		Helper.printCalculationTime();
	}

	private String keyToString(int[] key) {
		StringBuilder stringHelper = new StringBuilder();

		for (int element : key) {
			stringHelper.append((char) element);
		}

		return stringHelper.toString();
	}

	private StringBuilder translateMessage(int[] key) {
		int index = 0, offset = 0;
		StringBuilder res = new StringBuilder();

		while (index < this.message.size()) {
			res = res.append(this.getXor(this.message.get(index), key[offset]));

			if (offset == 2) {
				offset = 0;
			} else {
				offset++;
			}
			index++;
		}

		return res;
	}

	public char getXor(char c, int key) {
		return (char) (c ^ key);
	}
}
