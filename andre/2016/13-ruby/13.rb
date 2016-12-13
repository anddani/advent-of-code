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
  
  while not queue.empty?
    point = queue.shift
    if not visited.include? point
      visited.add(point)
      up    = [point[0], point[1]-1]
      down  = [point[0], point[1]+1]
      left  = [point[0]-1, point[1]]
      right = [point[0]+1, point[1]]

      if up[1] >= 0 and not visited.include? up and office[up[1]][up[0]] == '.'
        queue.push(up)
        steps[up] = steps[point] + 1
      end
      if down[1] <= office.count-1 and not visited.include? down and office[down[1]][down[0]] == '.'
        queue.push(down)
        steps[down] = steps[point] + 1
      end
      if left[0] >= 0 and not visited.include? left and office[left[1]][left[0]] == '.'
        queue.push(left)
        steps[left] = steps[point] + 1
      end
      if right[0] <= office.count-1 and not visited.include? right and office[right[1]][right[0]] == '.'
        queue.push(right)
        steps[right] = steps[point] + 1
      end
    end
  end
  return [steps[goal], steps.select{|k,v| v <= 50}.count]
end

layout.each_with_index {|row,y| row.replace(row.map.with_index{|val,x| office(x, y, num)})}

puts bfs(layout, [1, 1], [31, 39]).to_s
