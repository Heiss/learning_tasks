# https://www.reddit.com/r/dailyprogrammer/comments/52enht/20160912_challenge_283_easy_anagram_detector/

def anagramHash(word, check)
  if(word.is_a?(String) and check.is_a?(String))
    hash = Hash.new
    hash.default = 0

    word.split(//).each { |i|
      if(i != " ")
        hash[i] = hash[i] + 1
      end
    }

    check.split(//).each { |i|
      if(i != " ")
        hash[i] = hash[i] - 1
      end
    }

    equalWords = "";
    hash.each_value{ |v|
      if(v.is_a?(Integer) and v != 0)
        equalWords = "NOT ";
      end
    }

    puts "#{word} is #{equalWords}an anagram of #{check}"
  end
end

def anagramSort(word, check)
  if word.chars.sort.join == check.chars.sort.join
    puts "#{word} is an anagram of #{check}"
  else
    puts "#{word} is NOT an anagram of #{check}"
  end
end

def readFile()
  return File.open("values/283.txt", "r")
end

def start()
  readFile.readlines.each { |line|
    split = line.split(" ? ")
    split.map!{ |w|
      w.downcase.gsub(/[^a-zA-Z]/, '')
    }

    anagramHash(split[0], split[1])
    anagramSort(split[0], split[1])
  }
end

start