# https://www.reddit.com/r/dailyprogrammer/comments/55zdxx/20161005_challenge_286_intermediate_zeckendorf/

require_relative "helper/fib"

File.open("values/286im.txt", "r").readlines.each do |line|
  number = line.chomp.to_i
  result = Array.new
  tmpNr = number
  i = 1

  i += 1 while Fib.fibonacci(i) < number
  while tmpNr > 0
    if Fib.fibonacci(i) <= tmpNr
      tmpNr -= Fib.fibonacci(i)
      result.push(Fib.fibonacci(i))
    end

    i -= 1
  end

  puts "#{number} = #{result.to_a.join(" + ")}"
end

puts Fib.table