import Data.List
import Data.List.Split
import Text.Parsec

areaOfSmallest :: Int -> Int -> Int -> Int
areaOfSmallest x y z =
    let [a, b] = take 2 (sort [x, y, z])
    in a * b

calc :: String -> Int
calc line =
    let dims = splitOn "x" line
        l   = read . head $ dims
        w   = read $ dims !! 1
        h   = read $ dims !! 2
    in 2*l*w + 2*w*h + 2*l*h + areaOfSmallest l w h

solveOne :: [String] -> Int
solveOne dims = foldr ((+) . calc) 0 dims

main :: IO ()
main = do
    input <- readFile "inputs/2015-02"
    print $ solveOne (lines input)
