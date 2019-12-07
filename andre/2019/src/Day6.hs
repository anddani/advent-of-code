module Day6 where

import qualified Data.Set as S
import qualified Data.Map as M
import Data.List (unfoldr)
import Data.List.Split (splitOn)

type Inner = String
type Outer = String

type Orbit = M.Map Outer Inner

toTuple :: [a] -> (a, a)
toTuple [x, y] = (x, y)

dup :: a -> (a, a)
dup x = (x, x)

parseDirectOrbits :: [String] -> Orbit
parseDirectOrbits = M.fromList . map ((\[x, y] -> (y, x)) . splitOn ")")

orbitSequence :: Orbit -> Outer -> [Inner]
orbitSequence orbits outer =
  unfoldr (\o -> dup <$> M.lookup o orbits) outer

orbitCount :: Orbit -> Outer -> Int
orbitCount orbit outer = length $ orbitSequence orbit outer

part1 :: Orbit -> Int
part1 orbits =
  let keys = M.keys orbits
  in  sum $ (orbitCount orbits) <$> keys

part2 :: Orbit -> Int
part2 orbits =
  let myPath = S.fromList $ orbitSequence orbits "YOU"
      santasPath = S.fromList $ orbitSequence orbits "SAN"
      myUniquePath = S.difference myPath santasPath
      santasUniquePath = S.difference santasPath myPath
  in  S.size $ S.union myUniquePath santasUniquePath

main :: IO ()
main = do
  file <- readFile "./input/d6p1.input"
  let input = lines file
  let orbits = parseDirectOrbits input
  print $ part1 orbits
  print $ part2 orbits
