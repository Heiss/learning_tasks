# https://www.reddit.com/r/dailyprogrammer/comments/50hbtp/20160831_challenge_281_intermediate_dank_usernames/

hash = Hash.new
File.open("values/284.txt", "r").readlines.each {|line|
  File.open("values/281im.txt", "r").readlines.each {|name|
    if hash[name.chomp].nil?
      array = Array.new
      hash[name.chomp] = array
    end

    tempLine = line.to_s.chomp
    tempName = name.to_s.chomp.downcase

    while tempLine.length > 0
      index = tempName.index(tempLine.chars[0])
      if !index.nil?
        tempLine.slice!(0)
        tempName.slice!(0..index)
      else
        break
      end
    end

    if tempLine.length == 0
      hash[name.chomp].to_a.push(line.chomp)
    end
  }
}

hash.each do |name, values|
  maxName = ""
  values.each { |word|
    if word.to_s.length > maxName.length
      maxName = word.to_s
    end
  }

  puts "#{name} => #{maxName}"
end