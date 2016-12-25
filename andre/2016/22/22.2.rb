#!/usr/bin/env ruby

require_relative 'priority_queue'

$dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]]

class Node
  attr_accessor :id, :pos, :avail, :used, :x, :y

  def initialize(id, x, y, u, a)
    @id = id
    @x = x.to_i
    @y = y.to_i
    @pos = [@y, @x]
    @used = u.to_i
    @avail = a.to_i
  end

  def free?
    return @used == 0
  end

  def ==(other)
    @id == other.id
  end
end

class AStar
  def initialize(disks, ok)
    @disks = disks
    @ok = ok
    @largest_y = @disks.count - 1
    @largest_x = @disks[0].count - 1
  end

  def a_star(start, target)
    @start = start
    @pq = PriorityQueue.new
    @came_from = {}
    @g_score = {}

    @pq.insert(0, start)
    @came_from[start] = nil
    @g_score[start] = 0

    until @pq.empty?
      curr = @pq.min
      
      # Return the best node to go to
      if curr == target
        c_p = curr
        path = [curr]
        while c_p != @start
          c_p = @came_from[c_p]
          path << c_p
        end
        return path[-2]
      end

      $dirs.each do |dir|
        temp_node = curr.pos.zip(dir).map {|x| x.reduce(:+)}

        # Valid node?
        next if not temp_node[0].between?(0, @largest_y)
        next if not temp_node[1].between?(0, @largest_x)
        next_node = @disks[temp_node[0]][temp_node[1]]

        # Skip if this is the node we came from or not ok node
        next if @came_from.key? next_node or not @ok.include? next_node

        new_cost = @g_score[curr] + 1
        # if we have not visited this node or better score to node
        if (not @g_score.key? next_node) or new_cost < @g_score[next_node]
          @g_score[next_node] = new_cost
          @pq.insert(new_cost, next_node)
          @came_from[next_node] = curr
        end
      end
    end
  end
end

class Disks
  def initialize
    @disks = []
    @a = [0, 0]
  end

  def run
    c = 0
    until @a == @g
      empty = @disks.flatten.find{ |n| n.free? }
      target_node = @disks[@g[1]][@g[0]-1]
      g = @disks[@g[1]][@g[0]]

      if not target_node.free?
        ok_nodes = @disks.flatten.permutation(2).select do |p|
          not p[0].free? and p[0].used <= p[1].avail
        end.flatten.uniq - [g]

        alg = AStar.new(@disks, ok_nodes)

        best_node = alg.a_star(empty, target_node)

        # Move best_node -> empty
        empty.used += best_node.used
        empty.avail -= best_node.used
        best_node.avail += best_node.used
        best_node.used = 0
      else # Target node is empty
        # Move g -> target_node
        target_node.used += g.used
        target_node.avail -= g.used
        g.avail += g.used
        g.used = 0
        @g = [@g[0]-1, @g[1]]
      end
      c += 1
    end
    return c
  end

  def new_node(line)
    m = /(node-x(\d+)-y(\d+))\s*\d+T\s*(\d+)T\s*(\d+)T\s*\d+/.match(line)
    n = Node.new(m[1], m[2], m[3], m[4], m[5])
    @disks[n.y] ||= []
    @disks[n.y][n.x] = n
    @g = [n.x, 0] if n.y == 0
  end
end

disks = Disks.new
ARGF.readlines.each do |line|
  if /node/.match(line)
    disks.new_node(line)
  end
end
puts disks.run
