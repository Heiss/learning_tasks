package problem;

import java.math.BigInteger;

import helper.Helper;

public class App020 {
	final static long number = 100;

	public static void main(String[] args) {
		BigInteger fac = Helper.fac(number);
		System.out.println(fac + ", " + Helper.getSumOfDigits(fac));
	}
}
