# https://www.reddit.com/r/dailyprogrammer/comments/4z04vj/20160822_challenge_280_easy_0_to_100_real_quick/

File.open("values/280.txt", "r").readlines.each { |line|
  result = 0

  if line.to_s.chars[5].to_i == 1
    result += 5
  end

  if line.to_s.chars[4].to_i == 1
    result += 50
  end

  number = line.to_s.chomp

  invalidSeq = false
  nullFound = false
  number[6..9].to_s.each_char {|c|
    if nullFound and c.to_i == 1
      invalidSeq = true
      break
    end

    if c.to_i == 0
      nullFound = true
    end

    result += c.to_i
  }

  if !invalidSeq
    nullFound = false
    temp = 0
    number[0..3].to_s.reverse.each_char {|c|
      if nullFound and c.to_i == 1
        invalidSeq = true
        break
      end

      if c.to_i == 0
        nullFound = true
      end

      temp += c.to_i
    }
    result += temp * 10
  end

  puts "#{line.chomp} => #{(invalidSeq) ? "INVALID" : result}"
}