#!/usr/bin/env ruby

c = gets(nil).split.map(&:to_i)
puts (0..c.length).reduce{ |sum,i| sum += c.combination(i).to_a.select{|v| v.inject(&:+) == 150}.count }

combs = []

(1..c.length).each{|i| combs += c.combination(i).to_a }
best = Hash.new
combs.each do |item|
  best[item.count] ||= 0
  best[item.count]+=1 if item.inject(&:+) == 150
end
puts best.sort_by(&:last).select{|v| v[1] > 0}[0][1]
