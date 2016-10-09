# https://projecteuler.net/problem=28

t1 = Time.now

n = 1001

result = 1
i = 2
count = 0
1.step(n*n).to_a.each_with_index do |value, index|
  next if index == 0
  if count == 4
    count = 0
    i += 2
  elsif (index) % i == 0
    result += value
    count += 1
  end
end

puts result
puts "Time: #{Time.now - t1}"