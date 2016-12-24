#!/usr/bin/env ruby

require 'set'

map = ARGF.readlines.map(&:chomp).map(&:chars)
height = map.count
width = map[0].count

dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]]

num_numbers = 8

dist = Array.new(num_numbers) { Array.new(num_numbers) }

num_numbers.times do |i|
  start = []
  map.each_index { |row| col = map[row].index i.to_s ; start = [row, col] if col }
  queue = []
  visited = Set.new
  visited.add(start)
  queue.push([0, start])

  until queue.empty?
    steps, curr = queue.shift

    c = map[curr[0]][curr[1]]
    if /\d/.match(c)
      c = c.to_i
      dist[i][c] = steps
      dist[c][i] = steps
    end

    valid_neighbors = dirs.map do |d|
      curr.zip(d).map{|x| x.reduce(:+)}
    end.select do |n|
      n[0].between?(0, height-1) and
      n[1].between?(0, width-1) and
      map[n[0]][n[1]] != "#" and
      not visited.include? n
    end

    valid_neighbors.each do |n|
      visited.add(n)
      queue.push([steps + 1, n])
    end
  end
end

# Part 1
paths = (0..num_numbers-1).to_a.permutation(num_numbers).to_a.select { |e| e[0] == 0 }
best1 = Float::INFINITY
best2 = Float::INFINITY
paths.each do |path|
  best1 = [best1, path.each_cons(2).inject(0) { |sum, e| sum += dist[e[0]][e[1]] }].min
  best2 = [best2, path.push(0).each_cons(2).inject(0) { |sum, e| sum += dist[e[0]][e[1]] }].min
end

puts "Part 1: #{best1}"
puts "Part 2: #{best2}"
