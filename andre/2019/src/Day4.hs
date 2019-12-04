module Day4 where

import Data.Monoid
import Data.List

twoAdjacent :: Int -> Bool
twoAdjacent = any (uncurry (==)) . (zip <*> tail) . show

onlyIncrease :: Int -> Bool
onlyIncrease = all (uncurry (<=)) . (zip <*> tail) . show

smallGroups :: Int -> Bool
smallGroups = any ((== 2) . length) . group . show

solve :: [(Int -> Bool)] -> [Int] -> Int
solve predicates =
  let isValid = getAll . foldMap (All .) predicates
   in length . filter isValid

part1 :: [(Int -> Bool)]
part1 = [twoAdjacent, onlyIncrease]

part2 :: [(Int -> Bool)]
part2 = [smallGroups, onlyIncrease]

main :: IO ()
main = do
  let input = [124075..580769]
  print $ solve part1 input
  print $ solve part2 input
