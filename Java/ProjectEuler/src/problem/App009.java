package problem;

public class App009 {
	public static void main(String[] args) {
		int a = 1, b = 1, c = 1;
		int numbers = 1000;

		while (a < numbers) {
			b = 1;
			while (b < numbers) {
				c = 1;
				while (c < numbers) {
					if (isPythagorean(a, b, c) && a + b + c == numbers) {
						System.out.println(a + ", " + b + ", " + c);
						System.out.println(a * b * c);
						return;
					}
					c++;
				}
				b++;
			}
			a++;
		}
		System.out.println("finished");

	}

	public static boolean isPythagorean(int a, int b, int c) {
		if (Math.pow(a, 2) + Math.pow(b, 2) == Math.pow(c, 2)) {
			return true;
		}
		return false;
	}

}
