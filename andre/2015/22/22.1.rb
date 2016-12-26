#!/usr/bin/env ruby

require_relative 'priority_queue'

boss = { :hp => 58, :damage => 9, :poison_turns => 0 }
player = { :hp => 50, :mana => 500, :shield_turns => 0, :recharge_turns => 0, :spent_mana => 0 }
$spells = [[:missile, 53], [:drain, 73], [:shield, 113], [:poison, 173], [:recharge, 229]]

queue = PriorityQueue.new
queue.insert(0, [player, boss, :player])

until queue.empty?
  player, boss, turn = queue.min

  # Effects
  if boss[:poison_turns] > 0
    boss[:hp] -= 3
    boss[:poison_turns] -= 1
  end
  player[:armor] = 0
  if player[:shield_turns] > 0
    player[:armor] = 7
    player[:shield_turns] -= 1
  end
  if player[:recharge_turns] > 0
    player[:mana] += 101
    player[:recharge_turns] -= 1
  end

  # Check if dead
  if boss[:hp] <= 0
    puts player[:spent_mana]
    break
  end

  if turn == :player and player[:hp] > 0
    # Try each spell
    $spells.select do |spell|
      spell[1] <= player[:mana]
    end.each do |spell|
      new_player = player.dup
      new_boss = boss.dup
      new_player[:mana] -= spell[1]
      new_player[:spent_mana] += spell[1]

      if spell[0] == :missile
        new_boss[:hp] -= 4
      elsif spell[0] == :drain
        new_boss[:hp] -= 2
        new_player[:hp] += 2
      elsif spell[0] == :shield and new_player[:shield_turns] == 0
        new_player[:shield_turns] = 6
      elsif spell[0] == :poison and new_boss[:poison_turns] == 0
        new_boss[:poison_turns] = 6
      elsif spell[0] == :recharge and new_player[:recharge_turns] == 0
        new_player[:recharge_turns] = 5
      end
      queue.insert(new_player[:mana_spent], [new_player, new_boss, :boss])
    end
  elsif turn == :boss
    player[:hp] -= [boss[:damage] - player[:armor], 1].max
    queue.insert(player[:mana_spent], [player, boss, :player])
  end
end
