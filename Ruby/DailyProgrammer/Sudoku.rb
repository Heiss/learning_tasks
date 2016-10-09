# http://t-a-w.blogspot.de/2016/10/solving-sudoku-with-ruby-and-z3.html

require "z3"

data = File.read("values/Sudoku.txt").strip.split("\n").map do |line|
  line.split.map{|c| c == "_" ? nil : c.to_i}
end

solver = Z3::Solver.new

cells = (0..8).map do |j|
  (0..8).map do |i|
    Z3.Int("cell[#{i},#{j}]")
  end
end

cells.flatten.each do |v|
  solver.assert v >= 1
  solver.assert v <= 9
end

cells.each_with_index do |row, i|
  row.each_with_index do |var, j|
    solver.assert var == data[i][j] if data[i][j]
  end
end

cells.each do |row|
  solver.assert Z3.Distinct(*row)
end

cells.transpose.each do |column|
  solver.assert Z3.Distinct(*column)
end

cells.each_slice(3) do |rows|
  rows.transpose.each_slice(3) do |square|
    solver.assert Z3.Distinct(*square.flatten)
  end
end

raise "Failed to solve" unless solver.check == :sat
model = solver.model
cells.each do |row|
  puts row.map{|v| model[v]}.join(" ")
end

solver = Z3::Solver.new

def declare_abs(var, expr, s)
  s.assert Z3.Or(
      Z3.And(var == expr,  expr >= 0),
      Z3.And(var == -expr, expr <= 0)
  )
end

x = Z3.Real("x")
xm2abs = Z3.Real("|x-2|")
declare_abs(xm2abs, x-2, solver)
m6abs = Z3.Real("|-6|")
declare_abs(m6abs, -6, solver)
solver.assert x < 2
solver.assert xm2abs == 4*m6abs
raise "Failed to solve" unless solver.check == :sat
model = solver.model
model.each do |n, v|
  puts "#{n.to_s} = #{v.to_s}"
end

a = Z3.True
b = Z3.False
solver = Z3::Solver.new
puts "Checking if true == false"
solver.prove!(a==b)