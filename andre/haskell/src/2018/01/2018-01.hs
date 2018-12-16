import qualified Data.Set as S

removePlusSign :: String -> String
removePlusSign ('+':xs) = xs
removePlusSign xs = xs

solveOne :: [Int] -> Int
solveOne = sum

solveTwo :: [Int] -> Int
solveTwo numbers = firstVisitedSum
  where
    sums = scanl1 (+) $ cycle numbers
    members = scanl (flip S.insert) S.empty sums
    (firstVisitedSum, _) = head
      . dropWhile (\(sum, visited) -> not visited)
      $ zipWith (\sum visited -> (sum, S.member sum visited)) sums members

main :: IO ()
main = do
    input <- readFile "inputs/2018-01"
    let numbers = map (read . removePlusSign) $ lines input
    print $ solveOne numbers
    print $ solveTwo numbers
