package problem;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.Set;

public class App001 {
	public static final int numbers = 1000;
	static ArrayList<Integer> list;
	static Set<Integer> result;

	public static void main(String[] args) {
		new App001().run();
	}

	private void run() {
		list = new ArrayList<Integer>();

		int i = 0;
		while (i < numbers) {
			App001.list.add(i++);
		}

		this.firstAttempt();
	}

	private void firstAttempt() {
		App001.result = new HashSet<Integer>();

		for (Integer c : list) {
			if (c % 3 == 0 || c % 5 == 0) {
				result.add(c);
			}
		}

		int erg = 0;
		for (Integer c : result) {
			erg += c;
		}

		System.out.println(erg);
	}

}
