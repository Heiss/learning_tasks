# https://www.reddit.com/r/dailyprogrammer/comments/4uhqdb/20160725_challenge_277_easy_simplifying_fractions/
require_relative 'helper/gcd'

File.open("values/277.txt", "r").readlines.each { |line|
  split = line.chomp.split(" ")
  n1 = split[0].to_i
  n2 = split[1].to_i

  gcd = GCD.euclid(n1, n2).to_i
  while(gcd > 1)
    n1 /= gcd
    n2 /= gcd
    gcd = GCD.euclid(n1, n2)
  end

  puts "#{split[0]} #{split[1]} = #{n1} #{n2}"
}

puts "\nBonus:"
equations = Hash.new
lineCount = 1
equationsCount = 0
File.open("values/277b.txt", "r").readlines.each { |line|
  if lineCount == 1
    equationsCount = line.chomp.to_i
  elsif lineCount > 1 and lineCount <= 1 + equationsCount
    split = line.chomp.split(" ")
    equations[split[0]] = split[1]
  else
    split = line.chomp.split(" ")
    left = split[0]
    right = split[1]

    anyChanges = true
    while(anyChanges)
      anyChanges = false
      temp1 = ""
      left.chars.each { |w|
        if !equations[w].nil?
          temp1 = temp1 + equations[w]
          anyChanges = true
        else
          temp1 = temp1 + w
        end
      }

      temp2 = ""
      right.chars.each { |w|
        if !equations[w].nil?
          temp2 = temp2 + equations[w]
          anyChanges = true
        else
          temp2 = temp2 + w
        end
      }

      left = temp1
      right = temp2
    end

    temp1 = left.to_s
    temp2 = right.to_s

    left.chars.each { |w|
      if !right.index(w).nil?
        temp1.slice!(left.index(w))
        temp2.slice!(right.index(w))
      end
    }

    left = temp1
    right = temp2

    if left.empty?
      left = 1
    end

    if right.empty?
      right = 1
    end

    puts "#{left} #{right}"
  end

  lineCount += 1
}