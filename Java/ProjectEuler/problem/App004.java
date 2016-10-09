package problem;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;

import helper.Helper;

public class App004 {
	public static int digits = 3;
	private static int res;
	private static ArrayList<Integer> resList = new ArrayList<>();

	public static void main(String[] args) {
		String strBegin = "9";
		String strEnd = "1";

		for (int i = 1; i < digits; i++) {
			strBegin += "9";
			strEnd += "0";
		}
		System.out.println("### end generate begin " + strBegin + " and end " + strEnd);

		int startZ = Integer.valueOf(strBegin);
		int endZ = Integer.valueOf(strEnd);

		int zahl1 = startZ, zahl2 = startZ;

		while (zahl1 > endZ) {

			if (zahl2 == endZ) {
				zahl2 = startZ;
			}

			while (zahl2 > endZ) {
				res = zahl1 * zahl2;
				if (Helper.isPalindrome(BigInteger.valueOf(res))) {
					resList.add(res);
				}
				zahl2--;
			}
			zahl1--;
		}

		Collections.sort(resList, new Comparator<Integer>() {
			@Override
			public int compare(Integer o1, Integer o2) {
				return Integer.compare(o1, o2);
			}
		});

		System.out.println(resList.get(resList.size() - 1));
	}
}
