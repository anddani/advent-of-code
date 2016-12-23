#!/usr/bin/env ruby

require 'set'
require 'pp'

lines = ARGF.readlines.map{|l| /node-x(\d+)-y(\d+)\s*(\d+)T\s*(\d+)T\s*(\d+)T\s*(\d+)/.match(l) }.compact

nodes = {}
largest_x = 0

lines.each do |l|
  nodes[[l[2].to_i, l[1].to_i]] = {
    :size => l[3].to_i,
    :used => l[4].to_i,
    :avail => l[5].to_i,
    :p => l[6].to_i
  }
  largest_x = l[1].to_i if l[1].to_i > largest_x
end

num_col = largest_x+1

used = Array.new(0, num_col*num_col)
avail = Array.new(0, num_col*num_col)
nodes.each do |k, v|
  used[k[0]*num_col + k[1]] = v[:used]
  avail[k[0]*num_col + k[1]] = v[:avail]
end

visited = Set[[used, avail]]
# queue = [[[largest_x, 0], 0, nodes]]
queue = [[[0, largest_x], 0, used, avail]]
best = 0

# up, down, left, right
dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]]
positions = nodes.keys

curr_step = 0

until queue.empty?
  g, steps, u, a = queue.shift

  if steps == curr_step
    puts "Current step: #{steps}"
    curr_step += 1
  end

  if g == [0, 0]
    best = steps
    break
  end

  # For each position, try to go up down left right
  positions.each do |p|
    (0..3).each do |i|
      n = p.zip(dirs[i]).map{|x| x.reduce(:+)}
      # Valid move?
      if nodes.key?(n) and u[p[0]*num_col + p[1]] <= a[n[0]*num_col + n[1]]

        next_a = a.dup
        next_u = u.dup
        to_move = next_u[p[0]*num_col + p[1]]
        next_u[p[0]*num_col + p[1]] = 0
        next_a[p[0]*num_col + p[1]] += to_move
        next_u[n[0]*num_col + n[1]] += to_move
        next_a[n[0]*num_col + n[1]] -= to_move

        if not visited.include?([next_u, next_a])
          visited.add([next_u, next_a])
          # Move goal if current pos
          if p == g
            queue.push([n, steps+1, next_u, next_a])
          else
            queue.push([g, steps+1, next_u, next_a])
          end
        end
      end
    end
  end
end

puts best
