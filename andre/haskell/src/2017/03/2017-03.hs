import Data.List

getSteps :: Int -> Maybe (Int, Int)
getSteps input = find ((>= input) . snd) $ zip [0, 2..] [x * x | x <- [1, 3..]]

solveOne :: Int -> Int
solveOne input = steps - (edge - input) `mod` steps
    where Just (steps, edge) = getSteps input

main :: IO ()
main = do
    input <- readFile "inputs/2017-03"
    print $ solveOne (read input)
    -- print $ solveTwo input
