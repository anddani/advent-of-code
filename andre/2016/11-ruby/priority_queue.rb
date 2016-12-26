class PriorityQueue
  def initialize
    @elements = Hash.new { |h, k| h[k] = [] }
  end

  def insert(prio, data)
    @elements[prio] << data
  end

  def min
    return nil if @elements.empty?
    key, val = *@elements.min
    result = val.shift
    @elements.delete(key) if val.empty?
    return result
  end

  def include?(data)
    return @elements.values.flatten.include? data
  end

  def empty?
    return @elements.empty?
  end

  def size
    return @elements.size
  end
end
