import Data.List

solveOne :: String -> Int
solveOne input =
    let up = length $ filter (== '(') input
        down = length $ filter (== ')') input
    in up - down

main :: IO ()
main = do
    input <- readFile "inputs/2015-01-part1"
    print (solveOne input)
