# https://de.wikipedia.org/wiki/RSA-Kryptosystem#Verfahren

require_relative "gcd"

class RSA
  # return private key, public key, modulo
  def self.generateKeys digits
    raise "Cant use only 1 digit!" if digits == 1

    ord = digits - 1; p = 1; q = 1

    p = Random.rand(10**ord .. 10**(ord + 1) - 1) while !GCD.isPrime p or p == q or p == 1
    q = Random.rand(10**ord .. 10**(ord + 1) - 1) while !GCD.isPrime q or p == q or q == 1
    n = p * q

    phi = (p - 1) * (q - 1); e = phi
    e = Random.rand(2..phi - 1) while GCD.euclid(e, phi) > 1

    d, s, t = GCD.extendedEuclid(e, phi)
    return s % phi, e % phi, n
  end

  def self.encrypt m, k, mod
    m.chars.map(&:ord).map{|c| c ** k % mod}
  end

  def self.decrypt m, k, mod
    m.map{|c| c ** k % mod}.map(&:chr).join("")
  end
end