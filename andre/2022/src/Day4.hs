module Day4 (solution) where

import Data.List.Split (splitOn)

type Range = (Int, Int)

intersects1 :: (Range, Range) -> Bool
intersects1 ((x1, y1), (x2, y2)) = x1 >= x2 && y1 <= y2 || x1 <= x2 && y1 >= y2

intersects2 :: (Range, Range) -> Bool
intersects2 ((x1, y1), (x2, y2)) = x1 >= x2 && x1 <= y2 || x2 >= x1 && x2 <= y1

toRanges :: String -> (Range, Range)
toRanges l = (lowRange l, highRange l)
    where
        lowRange = _toRange . head . splitOn ","
        highRange = _toRange . last . splitOn ","

        digits :: String -> [Int]
        digits = map read . splitOn "-"
        
        _toRange :: String -> Range
        _toRange s = (head (digits s), last (digits s))

part1 :: [(Range, Range)] -> Int
part1 = length . filter intersects1

part2 :: [(Range, Range)] -> Int
part2 = length . filter intersects2

solution :: IO (Int, Int)
solution = do
    input <- readFile "input/day4.txt"
    let ranges = map toRanges (lines input)
    return (part1 ranges, part2 ranges)

