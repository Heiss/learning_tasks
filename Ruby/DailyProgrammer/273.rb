# https://www.reddit.com/r/dailyprogrammer/comments/4q35ip/20160627_challenge_273_easy_getting_a_degree/

hash = Hash.new
hash["rd"] = "* 180 / Math::PI"
hash["dr"] = "* Math::PI / 180"
hash["fc"] = "/ 1.8 - 32 / 1.8"
hash["cf"] = "* 1.8 + 32"
hash["kc"] = "- 273.15"
hash["ck"] = "+ 273.15"

File.open("values/273.txt", "r").readlines.each do |line|
  tempLine = line.chomp.to_s

  convert = tempLine[-2..-1]
  number = tempLine[0..-3].to_f

  print "#{number}#{convert[0]} = "
  if hash[convert].nil?
    puts "No candidate for conversion"
  else
    result = eval(number.to_s + hash[convert].to_s).to_f.round(2)
    puts "#{result}#{convert[1]}"
  end
end