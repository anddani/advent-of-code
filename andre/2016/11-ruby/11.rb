#!/usr/bin/env ruby

require_relative 'priority_queue'
require 'set'

lines = ARGF.readlines.map(&:chomp)
floors = []

floors = lines.map do |l|
  g = l.scan(/([\w\-]+) generator/).flatten.map{ |item| item[0].upcase+"G" }
  m = l.scan(/(\w+)-compatible microchip/).flatten.map{ |item| item[0].upcase+"M" }
  Set.new(g + m)
end

# Valid state is when there are matching generator and chips
# or 
def valid?(floor)
  chips, generators = floor.partition{ |item| item[1] == "M" }
  return true if generators.empty?

  # All chips has a matching generator
  return chips.all? do |c|
    generators.any? { |g| g[0] == c[0] }
  end
end

s = [0, floors]
num_elems = floors.reduce(0) { |sum, e| sum += e.size }
distance = Hash.new(Float::INFINITY)
distance[s] = 0
queue = PriorityQueue.new
queue.insert(0, s)

until queue.empty?
  curr = queue.min
  f, state = curr
  items = state[f]

  if f == 3 and items.count == num_elems
    puts distance[curr]
    p state
    break
  end

  moves = items.to_a.combination(2).to_a + items.to_a.combination(1).to_a + []

  moves.select! { |m| valid?(items-m) }

  [-1, 1].each do |dir|
    new_f = f+dir

    if new_f.between?(0, 3)

      moves.each do |move|
        new_floor = state[new_f] + move

        if valid?(new_floor)

          new_state = state.map(&:dup)
          new_state[f] = items - move
          new_state[new_f] = new_floor

          e = [new_f, new_state]
          new_cost = distance[curr] + 1
          if distance[e] == Float::INFINITY or new_cost < distance[e]
            distance[e] = new_cost
            prio = new_cost - new_state[3].count + 3*new_state[0].count + 2*new_state[1].count + new_state[2].count
            queue.insert(new_cost, e)
          end
        end
      end
    end
  end
end
