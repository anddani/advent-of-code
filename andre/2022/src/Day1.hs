module Day1 (day1) where


day1 :: IO (Int, Int)
day1 = do
    input <- readFile "input/test.txt"
    putStrLn input
    return (0, 0)

