package problem;

public class App012 {
	static final int numbers = 500;

	public static void main(String[] args) {
		long begin = System.currentTimeMillis();

		int res = 0;
		int i = 0;

		while (true) {
			res = triangleNumber(i);
			if (checkCountDivisors(res) >= numbers) {
				System.out.println(res);
				break;
			}
			i++;
		}

		long end = System.currentTimeMillis();
		System.out.println(end - begin + "ms");
	}

	private static int checkCountDivisors(int num) {
		int count = 0;

		for (int i = 1; i <= Math.sqrt(num); i++) {
			if ((num % i) == 0) {
				count += 2;
			}
		}

		return count;
	}

	private static int triangleNumber(int num) {
		int res = 0;
		for (int i = 1; i <= num; i++) {
			res += i;
		}
		return res;
	}

}
