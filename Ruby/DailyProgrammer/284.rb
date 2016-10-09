# https://www.reddit.com/r/dailyprogrammer/comments/53ijnb/20160919_challenge_284_easy_wandering_fingers/

class C284
  def self.readFile
    return File.open("values/284.txt", "r")
  end

  def self.start
    @@wordlist = readFile.readlines
    findWord("qwertyuytresdftyuioknn")

    @@wordlist = readFile.readlines
    findWord("gijakjthoijerjidsdfnokg")
  end

  def self.lcs(word1, word2)
    hash = Hash.new
    hash.default = 0

    m = word1.to_s.length
    n = word2.to_s.length

    for i in 1..m
      for j in 1..n
        if word1.chars[i] == word2.chars[j]
          hash[[i, j]] = hash[[i - 1, j - 1]] + 1
        else
          hash[[i, j]] = [hash[[i, j - 1]], hash[[i - 1, j]]].max
        end
      end
    end

    return hash[[m, n]]
  end

  def self.findWord(word)
    print "#{word} => "

    @@wordlist.map!(&:chomp)
    @@wordlist.select! do |w| w.chars[0] == word.chars[0] and w.chars[w.to_s.length - 1] == word.chars[word.length - 1] end

    @@wordlist.each { |l|
      lcsLength = lcs(l.to_s, word.to_s)
      if l.to_s.length >= 5 and lcsLength == l.to_s.length
        print "#{l} #{lcsLength} "
      end
    }
    print "\n"
  end

  def self.findWords2(input)
    print "#{input} => "
    first_char = input.chars[0]
    last_char = input.chars[input.size - 1]

    candidates =  @@words.select{ |word| word.start_with?(first_char)}
    candidates.select!{ |word| word.end_with?(last_char) }

    candidates.reject! do |word|
      index = 0
      word.chars.any? do |char|
        index = input.index(char, index)
        index.nil?
      end
    end

    print "#{candidates}"
    print "\n"
  end

  def self.start2
    @@words = []

    File.open('values/284.txt', 'r') do |file|
      @@words = file.readlines.collect { |line| line.chomp }
      @@words.reject! { |word| word.size < 5 }
    end

    findWords2("qwertyuytresdftyuioknn")
    findWords2("gijakjthoijerjidsdfnokg")
  end
end

C284.start
C284.start2