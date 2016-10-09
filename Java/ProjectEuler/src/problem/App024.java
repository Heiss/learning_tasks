package problem;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;

public class App024 {
	final static String number = "0123456789";
	private static int i = 1;

	public static void main(String[] args) {
		// firstAttempt();
		secondAttempt(number);
	}

	@SuppressWarnings("unused")
	private static void firstAttempt() {
		StringBuilder str = new StringBuilder(number);
		ArrayList<StringBuilder> list = perm(str);

		Collections.sort(list, new Comparator<StringBuilder>() {
			@Override
			public int compare(StringBuilder o1, StringBuilder o2) {
				Long zahl1 = Long.parseLong(o1.toString());
				Long zahl2 = Long.parseLong(o2.toString());

				return zahl1.compareTo(zahl2);
			}
		});

		System.out.println(list.get((int) Math.pow(10, 6)));
	}

	public static void secondAttempt(String str) {
		permutation("", str);
	}

	private static void permutation(String prefix, String str) {
		int n = str.length();

		if (n == 0) {
			System.out.println("#" + i + "; " + prefix);
			if (i++ == 1000000) {
				System.exit(0);
			}
		} else {
			for (int i = 0; i < n; i++) {
				permutation(prefix + str.charAt(i), str.substring(0, i) + str.substring(i + 1, n));
			}
		}
	}

	public static ArrayList<StringBuilder> perm(StringBuilder pr) {
		ArrayList<StringBuilder> list = new ArrayList<>();
		int max = pr.length() - 1;

		for (int i = max; i >= 0; i--) {
			for (int j = max; j >= 0; j--) {
				pr = swap(pr, i, j);
				list.add(pr);
			}
		}

		return list;
	}

	public static StringBuilder swap(StringBuilder sw, int index1, int index2) {
		char help = sw.charAt(index1);

		sw.setCharAt(index1, sw.charAt(index2));
		sw.setCharAt(index2, help);

		return sw;

	}
}
