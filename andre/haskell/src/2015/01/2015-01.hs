import Data.List

solveOne :: String -> Int
solveOne input =
    let up = length $ filter (== '(') input
        down = length $ filter (== ')') input
    in up - down

solveTwo :: String -> Int -> Int -> Int
solveTwo [] _ _ = -1
solveTwo _ (-1) steps = steps - 1
solveTwo (x:rest) floor steps
    | x == '('  = solveTwo rest (floor + 1) (steps + 1)
    | x == ')'  = solveTwo rest (floor - 1) (steps + 1)
    | otherwise = -1

main :: IO ()
main = do
    input <- readFile "inputs/2015-01"
    print (solveOne input)
    print (solveTwo input 1 0)
