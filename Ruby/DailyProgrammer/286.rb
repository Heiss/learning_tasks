# https://www.reddit.com/r/dailyprogrammer/comments/55nior/20161003_challenge_286_easy_reverse_factorial/

File.open("values/286.txt", "r").readlines.each { |l|
  number = l.chomp.to_i
  fac = 1
  current = 1

  print "#{number} = "

  while(fac < number)
    current += 1
    fac *= current
  end

  if(fac == number)
    print "#{current}"
  else
    print "NONE"
  end

  print "!\n"
}