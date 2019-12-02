module Day1 (solve1) where

toInt :: String -> Int
toInt s = read s :: Int

fuel :: Int -> Int
fuel = (subtract 2) . (`div` 3)

recursiveFuel :: Int -> Int
recursiveFuel f
  | fuel f <= 0 = 0
  | otherwise = fuel f + (recursiveFuel $ fuel f)

solve :: [String] -> (Int -> Int) -> Int
solve xs f = sum $ map (f . toInt) xs

solve1 :: IO ()
solve1 = do
  file <- readFile "./input/d1p1.input"
  print $ solve (words file) fuel

solve2 :: IO ()
solve2 = do
  file <- readFile "./input/d1p1.input"
  print $ solve (words file) recursiveFuel
