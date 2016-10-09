# https://www.reddit.com/r/dailyprogrammer/comments/4dccix/20160404_challenge_261_easy_verifying_3x3_magic/

def isMagicSquare(values)
  root = Math::sqrt(values.length).to_i
  constant = root * (root ** 2 + 1) / 2

  hash = Hash.new
  hash.default = 0
  values.each_with_index do |integer, index|
    column = index % root
    row = (index / root).floor

    hash[["row", row]] = hash[["row", row]] + integer
    hash[["column", column]] = hash[["column", column]] + integer

    hash[["diag", 1]] = hash[["diag", 1]] + integer if row == column
    hash[["diag", 2]] = hash[["diag", 2]] + integer if row == root - 1 - column
  end

  return hash.values.map{|value| value == constant}.select{|value| value != true}.length == 0
end

File.open("values/261.txt", "r").readlines.each do |line|
  split = line.chomp.to_s.split(" ").map(&:to_i)

  # Bonus 2
  if Math::sqrt(split.length).to_i ** 2 < split.length
    puts "#{split} => #{(1.step(9).to_a - split).
        permutation.to_a.map{|value| isMagicSquare(split + value)}.
        select{|value| value == true}.length > 0}"
  else
    puts "#{split} => #{isMagicSquare(split)}"
  end
end