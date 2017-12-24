import Data.List.Split

type Coord = (Int, Int)

coordAdd :: Coord -> Coord -> Coord
coordAdd (l1, r1) (l2, r2) = (l1 + l2, r1 + r2)

toCoord :: String -> Coord
toCoord "s"  = (0, 1)
toCoord "sw" = (1, 1)
toCoord "nw" = (1, 0)
toCoord "n"  = (0, -1)
toCoord "ne" = (-1, -1)
toCoord "se" = (-1, 0)

cubeDistance :: Coord -> Int
cubeDistance (x, y) = maximum $ abs <$> [x, y, x-y]

solve :: [String] -> [Int]
solve directions =
    let coords = map toCoord directions
     in map cubeDistance $ scanl1 coordAdd coords

solveOne :: [String] -> Int
solveOne = last . solve

solveTwo :: [String] -> Int
solveTwo = maximum . solve

main :: IO ()
main = do
    input <- readFile "inputs/2017-11"
    let directions = splitOn "," (init input)
    print $ solveOne directions
    print $ solveTwo directions
