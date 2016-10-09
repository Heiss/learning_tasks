require_relative "helper/RSA"

m = "Test text was geeeht aaaaab!"

private, public, modulo = RSA.generateKeys 3
puts "Private: #{private} Public: #{public} Modulo: #{modulo}"

encMsg = RSA.encrypt m, public, modulo
decMsg = RSA.decrypt encMsg, private, modulo

puts "equal: #{m == decMsg} <=> #{m} \n=> #{encMsg} \n=> #{decMsg}"