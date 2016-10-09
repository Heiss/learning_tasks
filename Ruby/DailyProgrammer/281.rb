# https://www.reddit.com/r/dailyprogrammer/comments/504rdh/20160829_challenge_281_easy_something_about_bases/

hash = Hash.new
hash["a"] = 10
hash["b"] = 11
hash["c"] = 12
hash["d"] = 13
hash["e"] = 14
hash["f"] = 15

bases = Array.new
for i in 1..16
  bases.push(i)
end

File.open("values/281.txt", "r").readlines.each { |line|
  number = line.chomp.split("")
  puts "\nNumber: #{line}"

  number.map! {|w|
    (!w.is_a?(Integer) and !hash[w].nil?) ? hash[w] : w
  }

  bases.each { |base|
    count = 0
    result = 0
    skipped = false
    number.reverse.each { |c|
      if c.to_i < base
        result += c.to_i * (base ** count)
        count += 1
      else
        skipped = true
      end
    }

    if !skipped
      puts "base #{base} => #{result}"
      # break
    end
  }
}