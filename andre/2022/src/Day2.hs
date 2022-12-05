module Day2 (solution) where

shapeScore1 :: String -> Int
shapeScore1 [_, _, 'X'] = 1
shapeScore1 [_, _, 'Y'] = 2
shapeScore1 [_, _, 'Z'] = 3
shapeScore1 s = error $ "Did not find player's move" ++ s

shapeScore2 :: String -> Int
shapeScore2 "A X" = 3
shapeScore2 "A Y" = 1
shapeScore2 "A Z" = 2
shapeScore2 "B X" = 1
shapeScore2 "B Y" = 2
shapeScore2 "B Z" = 3
shapeScore2 "C X" = 2
shapeScore2 "C Y" = 3
shapeScore2 "C Z" = 1
shapeScore2 s = error $ "Did not find player's move" ++ s

outcome1 :: String -> Int
outcome1 "A X" = 3
outcome1 "A Y" = 6
outcome1 "A Z" = 0
outcome1 "B X" = 0
outcome1 "B Y" = 3
outcome1 "B Z" = 6
outcome1 "C X" = 6
outcome1 "C Y" = 0
outcome1 "C Z" = 3
outcome1 s = error $ "Unexpected round" ++ s

outcome2 :: String -> Int
outcome2 [_, _, 'X'] = 0
outcome2 [_, _, 'Y'] = 3
outcome2 [_, _, 'Z'] = 6
outcome2 s = error $ "Unexpected round" ++ s

runRounds :: (String -> Int) -> (String -> Int) -> [String] -> Int
runRounds outcome shapeScore rounds = shapeTotal + outcomeTotal
    where
        shapeTotal :: Int
        shapeTotal = sum $ map shapeScore rounds

        outcomeTotal :: Int
        outcomeTotal = sum $ map outcome rounds


part1 :: [String] -> Int
part1 = runRounds outcome1 shapeScore1

part2 :: [String] -> Int
part2 = runRounds outcome2 shapeScore2

solution :: IO (Int, Int)
solution = do
    input <- readFile "input/day2.txt"
    let rounds = lines input
    return (part1 rounds, part2 rounds)

