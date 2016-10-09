require 'yaml'

class GCD
  @hash = File.exists?("gcdhash.yml") ? YAML::load_file("gcdhash.yml") : Hash.new

  def self.saveHash
    File.open("gcdhash.yml", "w") do |file|
      file.write @hash.to_yaml
    end
  end

  def self.euclid a, b
    if @hash[[a, b]] == nil
      # if a == 0
      #   @hash[[n1, n2]] = b.to_i
      #   return b.to_i
      # else
      #   a > b ? a -= b : b -= a while (b != 0)
      # end
      #
      # @hash[[n1, n2]] = a.to_i
      # return a.to_i

      if b == 0
        @hash[[a, b]] = a
        return a
      end
      res = euclid(b, a % b)
      @hash[[b, a % b]] = res
      return res
    end
    return @hash[[a, b]]
  end

  def self.extendedEuclid a, b
    if b == 0
      return a, 1, 0
    end
    ds, ss, ts = extendedEuclid(b, a % b)

    d = ds
    s = ts
    t = ss - (a / b) * ts
    return d, s, t
  end

  def self.isPrime a
    if hash[a] == nil
      for i in 1..(Math.sqrt(a).ceil)
        if euclid(a, i) > 1
          @hash[a] = false
          return false
        end
      end
      @hash[a] = true
      return true
    end
    return @hash[a]
  end

  def self.hash
    @hash
  end
end