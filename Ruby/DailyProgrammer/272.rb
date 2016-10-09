# https://www.reddit.com/r/dailyprogrammer/comments/4oylbo/20160620_challenge_272_easy_whats_in_the_bag/

hash = Hash.new
File.open("values/scrabble.txt").readlines.each do |line|
  split = line.chomp.to_s.split("\t")

  hash[[split[0].to_s, "count"]] = split[1].to_i
  hash[[split[0].to_s, "value"]] = split[2].to_i
end

File.open("values/272.txt", "r").readlines.each do |line|
  tempHash = hash.clone
  puts "#{line.chomp}:"
  
  line.chomp.to_s.each_char {|c| tempHash[[c, "count"]] = tempHash[[c, "count"]].to_i - 1}
  tempHash = tempHash.select{|key, value| key[1] == "count"}.sort_by{|fst, value| value.to_i}.reverse

  countHash = Hash.new
  tempHash.each do |key, value|
    countHash[value] = Array.new if countHash[value].nil?
    countHash[value].to_a.push(key[0])
  end

  if countHash.map{|key, value| key.to_i < 0}.select{|value| value == true}.length > 0
    puts "Invalid input. More X's have been taken from the bag than possible."
    next
  else
    puts "####### Still in Bag #######"
    countHash.each {|key, value| puts "#{key}: #{value.sort_by { |value| value}}"}
  end

  number1 = countHash.reduce(0) {|total, (key, value)| total += key * value.to_a.length}
  tempHash.map! {|key, value| [key, hash[key] - value]}

  countHash = Hash.new
  tempHash.each do |key, value|
    countHash[value] = Array.new if countHash[value].nil?
    countHash[value].to_a.push(key[0])
  end

  puts "####### In Play #######"
  countHash.sort.reverse.each {|key, value| puts "#{key}: #{value.sort}"}

  number2 = countHash.reduce(0) {|total, (key, value)| total += key * value.to_a.length}
  puts "####### Values ####### \nInBag: #{number1} InPlay: #{number2}\n"
end