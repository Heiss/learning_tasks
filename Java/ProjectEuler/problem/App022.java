package problem;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;

import helper.Helper;

public class App022 {
	final static int num = 10000;

	public static void main(String[] args) {
		new App022().run();
	}

	private ArrayList<String> names = new ArrayList<>();

	private void run() {
		File f = new File("./src/ressources/prob22.txt");
		BufferedReader reader;
		String list = "";

		try {
			StringBuffer contents = new StringBuffer();
			String text = null;
			reader = new BufferedReader(new FileReader(f));
			while ((text = reader.readLine()) != null) {
				contents.append(text).append(System.getProperty("line.separator"));
			}
			list = contents.toString();
		} catch (FileNotFoundException e) {
			e.printStackTrace();
		} catch (IOException e) {
			e.printStackTrace();
		}

		String temp = "";
		boolean IsName = false;
		int i = 0;

		while (i < list.length()) {
			if (list.charAt(i) == '\"') {
				IsName = !IsName;
				i++;
			}
			if (IsName) {
				temp += list.charAt(i);
			} else {
				if (temp == "") {
					break;
				} else {
					this.names.add(temp);
					temp = "";
				}
			}
			i++;
		}
		Collections.sort(this.names);

		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();

		int i = 0;
		long sum = 0;

		for (String string : this.names) {
			sum += ++i * calcValue(string);
		}

		System.out.println(sum);

		Helper.printCalculationTime();
	}

	private static long calcValue(String tmp) {
		int LocalSum = 0;

		for (int j = 0; j < tmp.length(); j++) {
			if (tmp.charAt(j) == 'A') {
				LocalSum += 1;
			} else if (tmp.charAt(j) == 'B') {
				LocalSum += 2;
			} else if (tmp.charAt(j) == 'C') {
				LocalSum += 3;
			} else if (tmp.charAt(j) == 'D') {
				LocalSum += 4;
			} else if (tmp.charAt(j) == 'E') {
				LocalSum += 5;
			} else if (tmp.charAt(j) == 'F') {
				LocalSum += 6;
			} else if (tmp.charAt(j) == 'G') {
				LocalSum += 7;
			} else if (tmp.charAt(j) == 'H') {
				LocalSum += 8;
			} else if (tmp.charAt(j) == 'I') {
				LocalSum += 9;
			} else if (tmp.charAt(j) == 'J') {
				LocalSum += 10;
			} else if (tmp.charAt(j) == 'K') {
				LocalSum += 11;
			} else if (tmp.charAt(j) == 'L') {
				LocalSum += 12;
			} else if (tmp.charAt(j) == 'M') {
				LocalSum += 13;
			} else if (tmp.charAt(j) == 'N') {
				LocalSum += 14;
			} else if (tmp.charAt(j) == 'O') {
				LocalSum += 15;
			} else if (tmp.charAt(j) == 'P') {
				LocalSum += 16;
			} else if (tmp.charAt(j) == 'Q') {
				LocalSum += 17;
			} else if (tmp.charAt(j) == 'R') {
				LocalSum += 18;
			} else if (tmp.charAt(j) == 'S') {
				LocalSum += 19;
			} else if (tmp.charAt(j) == 'T') {
				LocalSum += 20;
			} else if (tmp.charAt(j) == 'U') {
				LocalSum += 21;
			} else if (tmp.charAt(j) == 'V') {
				LocalSum += 22;
			} else if (tmp.charAt(j) == 'W') {
				LocalSum += 23;
			} else if (tmp.charAt(j) == 'X') {
				LocalSum += 24;
			} else if (tmp.charAt(j) == 'Y') {
				LocalSum += 25;
			} else if (tmp.charAt(j) == 'Z') {
				LocalSum += 26;
			}
		}

		return LocalSum;
	}
}
