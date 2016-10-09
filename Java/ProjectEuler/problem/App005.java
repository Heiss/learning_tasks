package problem;

public class App005 {
	public static final int numbers = 20;

	public static void main(String[] args) {
		int resN = 1;
		while (true) {
			if (isSmallestMultiple(resN)) {
				break;
			}
			resN++;
		}
		System.out.println(resN);
	}

	private static boolean isSmallestMultiple(int resN) {
		for (int i = 1; i <= numbers; i++) {
			if (resN % i != 0) {
				return false;
			}
		}
		return true;
	}
}
