#!/usr/bin/env ruby
require 'set'

num = ARGF.read.to_i
layout = Array.new(num) { Array.new(num, '.') }

def office(x, y, num)
  res = x*x + 3*x + 2*x*y + y + y*y + num
  num_bits = res.to_s(2).count("1")
  return num_bits.even? ? '.' : '#'
end

def bfs(office, start, goal)
  visited = Set[]
  steps = { start => 0 }
  queue = [start]

  dir = [[0, 1], [1, 0], [0, -1], [-1, 0]]
  
  until queue.empty?
    point = queue.shift
    if not visited.include? point
      visited.add(point)
      dir.each do |d|
        p = [point[0]+d[0], point[1]+d[1]]
        if p[0].between?(0, office.count-1) and
           p[1].between?(0, office.count-1) and
           not visited.include?(p) and
           office[p[1]][p[0]] == '.'
          queue.push(p)
          steps[p] = steps[point] + 1
        end
      end
    end
  end
  return [steps[goal], steps.select{|k,v| v <= 50}.count]
end

layout.each_with_index {|row,y| row.replace(row.map.with_index{|val,x| office(x, y, num)})}

puts bfs(layout, [1, 1], [31, 39]).to_s
