# https://de.wikipedia.org/wiki/Mersenne-Primzahl

require_relative "helper/gcd"

t1 = Time.now

def mersenne_prime n
  result = 2 ** n - 1
  return result, GCD.isPrime(result)
end

n = 0
arr = Array.new
while n <= 61
  res, isPrime = mersenne_prime(n)
  arr.push(res) if isPrime == true
  puts "n = #{n} => Mersenne-Number: #{res} => isPrime: #{isPrime}"
  n += 1
end

GCD.saveHash
puts " => saved"
puts "Result: #{arr - [0, 1]}"
# puts "Hash: #{GCD.hash}"
puts "Runtime: #{Time.now - t1}"