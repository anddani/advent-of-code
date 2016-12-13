class PriorityQueue
  attr_accessor :elements

  def initialize
    @elements = [nil]
  end

  def <<(element)
    @elements << element
    b_up(@elements.size - 1)
  end

  def b_up(index)
    p_index = index/2
    return if index <= 1 or @elements[p_index] >= @elements[index]
    exchange(index, p_index)
    b_up(p_index)
  end

  def exchange(s, t)
    @elements[s], @elements[t] = @elements[t], @elements[s]
  end

  def pop
    exchange(1, @elements.size - 1)
    max = @elements.pop
    b_down(1)
    return max
  end

  def b_down(index)
    child_index = (index*2)
    return if child_index > @elements.size - 1
    is_not_last = child_index < @elements.size - 1
    left_elem = @elements[child_index]
    right_elem = @elements[child_index + 1]
    child_index += 1 if is_not_last and right_elem > left_elem
    return if @elements[index] >= @elements[child_index]
    exchange(index, child_index)
    b_down(child_index)
  end
end
