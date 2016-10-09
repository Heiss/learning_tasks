# https://www.reddit.com/r/dailyprogrammer/comments/5196fi/20160905_challenge_282_easy_unusual_bases/

require_relative 'helper/fib.rb'

def readFile()
  return File.open("values/282.txt", "r")
end

def unusualBase(base, number)
  if(base.is_a?(String) and base == "10")
    current = number
    count = ""

    i = 1
    while(Fib.fibonacci(i) <= current)
      i += 1
    end
    i -= 1

    while(current > 0)
      fib = Fib.fibonacci(i)

      if(fib <= current)
        current -= fib
        count += "1"
      else
        count += "0"
      end

      i -= 1
    end

    for i in 1..i
      count += "0"
    end

    puts "#{base} #{number} => #{count}"
  elsif(base.is_a?(String) and base == "F")
    i = 1
    count = 0
    number.to_s.reverse.each_char{|c|
      if(c.to_i == 1)
        count += Fib.fibonacci(i)
      end

      i += 1
    }
    puts "#{base} #{number} => #{count}"
  end
end

def start()
  readFile.readlines.each{ |l|
    split = l.split(" ")
    unusualBase(split[0], split[1].to_i)
  }
end

start