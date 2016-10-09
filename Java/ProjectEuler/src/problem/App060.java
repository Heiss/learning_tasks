package problem;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

import com.google.common.collect.HashBasedTable;

import helper.Helper;

public class App060 {
	ArrayList<ArrayList<Integer>> resList = new ArrayList<>();
	HashBasedTable<Integer, Integer, Boolean> table = HashBasedTable.create();

	public static void main(String[] args) {
		new App060().run();
	}

	private int limit = 1000;
	private int num = 4;

	private void run() {
		this.firstAttempt();
	}

	private void firstAttempt() {
		Helper.timerStart();
		int lowestInt = Integer.MAX_VALUE;

		for (int i = 1; i <= this.limit; i++) {
			if (Helper.isPrime(i)) {
				this.table.put(i, i, false);
			}
		}

		System.out.println("-Find all primes");
		for (Integer row : this.table.rowKeySet()) {
			for (Integer column : this.table.columnKeySet()) {
				if (row != column) {
					boolean status = this.isConcatPrime(row, column);
					this.table.put(row, column, status);
					this.table.put(column, row, status);
				}
			}
		}

		System.out.println("-Checked all concat primes");
		for (Integer columnKey : this.table.columnKeySet()) {
			ArrayList<Integer> list = this.getColumnList(columnKey, this.table.column(columnKey));
			this.intersect(list);
		}

		for (ArrayList<Integer> arrayList : this.resList) {
			int sum = 0;
			for (Integer integer : arrayList) {
				sum += integer;
			}

			if (sum < lowestInt) {
				lowestInt = sum;
			}
		}

		System.out.println(this.resList);
		System.out.println(lowestInt);
		Helper.printCalculationTime();
	}

	private void intersect(List<Integer> listA) {
		for (Integer integer : listA) {
			ArrayList<Integer> listB = this.getColumnList(integer, this.table.column(integer));

			if (listA.containsAll(listB)) {
				continue;
			}

			ArrayList<Integer> intersect = this.intersect(listA, listB);

			if (intersect.size() == this.num) {
				this.resList.add(intersect);
				return;
			}

			if (intersect.size() < this.num) {
				return;
			}

			this.intersect(intersect);
		}
	}

	private ArrayList<Integer> intersect(List<Integer> listA, List<Integer> listB) {
		ArrayList<Integer> rtnList = new ArrayList<Integer>();
		for (Integer dto : listA) {
			if (listB.contains(dto)) {
				rtnList.add(dto);
			}
		}
		return rtnList;
	}

	private ArrayList<Integer> getColumnList(Integer columnKey, Map<Integer, Boolean> columnSet) {
		ArrayList<Integer> res = new ArrayList<>();
		res.add(columnKey);

		for (Integer rowKey : columnSet.keySet()) {
			if (this.table.get(rowKey, columnKey) != null && this.table.get(rowKey, columnKey)
					|| this.table.get(columnKey, rowKey) != null && this.table.get(columnKey, rowKey)) {
				res.add(rowKey);
			}
		}

		return res;
	}

	private boolean isConcatPrime(int a, int b) {
		return Helper.isPrime(String.valueOf(a) + String.valueOf(b))
				&& Helper.isPrime(String.valueOf(b) + String.valueOf(a));
	}
}
