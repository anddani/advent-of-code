#!/usr/bin/env ruby

require_relative 'priority_queue'
require 'set'

lines = ARGF.readlines.map(&:chomp)
floors = []

floors = lines.map do |l|
  g = l.scan(/([\w\-]+) generator/).flatten.map{ |item| item[0].upcase+"G" }
  m = l.scan(/(\w+)-compatible microchip/).flatten.map{ |item| item[0].upcase+"M" }
  g + m
end

# Valid state is when there are matching generator and chips
# or 
def valid?(state)
  state.each do |f|
    chips = f.select{ |item| item[1] == "M" }
    generators = f.select{ |item| item[1] == "G" }
    chips.each do |c|
      if not generators.include?(c[0]+"G") and generators.count > 0
        return false
      end
    end
  end
  return true
end

num_elems = floors.flatten.count
visited = Set[floors]
queue = [[0, 0, floors]]
best = 0

until queue.empty?
  curr_floor, steps, state = queue.shift
  if state[3].count == num_elems
    best = steps
    break
  end

  state.each_with_index do |f, i|
    combinations = f.combination(1).to_a + f.combination(2).to_a
    [1, -1].each do |dir|
      if (i+dir).between?(0, 3)
        combinations.each do |c|
          new_state = state.dup
          new_state[i] -= c
          new_state[i+dir] += c
          if valid? new_state
            visited.add(new_state)
            queue.push([i, (i-curr_floor).abs + steps+1, new_state])
          end
        end
      end
    end
  end
end

puts best
