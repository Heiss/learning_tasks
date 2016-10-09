package problem;

import java.math.BigInteger;
import java.util.ArrayList;

import helper.Helper;

public class App029 {
	private static int startA = 2, startB = 2;
	private static int endA = 100, endB = 100;

	public static void main(String[] args) {
		Helper.timerStart();

		ArrayList<String> list = new ArrayList<>();
		BigInteger res = BigInteger.ZERO;

		for (int i = startA; i <= endA; i++) {
			for (int j = startB; j <= endB; j++) {
				res = BigInteger.valueOf(i).pow(j);
				if (!list.contains(res.toString())) {
					list.add(res.toString());
				}
			}
		}

		System.out.println(list.size());
		Helper.printCalculationTime();
	}
}
