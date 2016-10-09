package problem;

public class App006 {
	public final static int numbers = 100;

	public static void main(String[] args) {
		int zahl1 = squareOfNumbers();
		int zahl2 = squareOfSumNumbers();

		System.out.println(zahl2 - zahl1);
	}

	private static int squareOfNumbers() {
		int n = 0;
		for (int i = 1; i <= numbers; i++) {
			n += Math.pow(i, 2);
		}
		return n;
	}

	private static int squareOfSumNumbers() {
		int n = 0;
		for (int i = 1; i <= numbers; i++) {
			n += i;
		}
		return (int) Math.pow(n, 2);
	}

}
