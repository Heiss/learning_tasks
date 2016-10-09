class Fib
  @@hash = Hash.new
  @@hash.default = 0

  def self.fibonacci(i)
    @@hash[i] = 1 if i <= 2
    @@hash[i] = fibonacci(i - 1) + fibonacci(i - 2) if @@hash[i] == 0
    return @@hash[i]
  end

  def self.table
    @@hash
  end
end