package problem;

public class App014 {
	final static int numbers = (int) Math.pow(10, 6);

	public static void main(String[] args) {
		long cur, count, longest = 0L, num = 0L;

		for (int i = 1; i < numbers; i++) {
			cur = i;
			count = 1;

			while (cur != 1) {
				cur = iterativeSequence(cur);
				count++;
			}

			if (count > longest) {
				longest = count;
				num = i;
			}
		}

		System.out.println("number with longest chain: " + num + " with length " + longest);
	}

	public static long iterativeSequence(long cur) {
		if (cur % 2 == 0) {
			return cur / 2;
		} else {
			return 3 * cur + 1;
		}
	}
}
