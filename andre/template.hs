module AdventOfCode where
import Data.List

-- Read data
one :: IO ()
one = do
    input <- readFile "in"
    print $ solveOne input
