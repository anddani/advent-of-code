import Data.List
import qualified Data.Map as M

data Instruction = Instruction Int (M.Map Int Int) (Int -> Int)

increment :: Int -> Instruction -> Instruction
increment nextPos (Instruction n table p) =
    Instruction nextPos (M.insert n (p x) table) p
        where Just x = M.lookup n table

next :: Instruction -> Maybe (Int, Instruction)
next curr@(Instruction n table _)
  | n >= 0 && n < M.size table = Just (nextPos, increment nextPos curr)
  | otherwise = Nothing
  where Just distance = M.lookup n table
        nextPos = n + distance

solveOne :: M.Map Int Int -> Int
solveOne table = length . unfoldr next $ Instruction 0 table (+1)

solveTwo :: M.Map Int Int -> Int
solveTwo table = length . unfoldr next $ Instruction 0 table (\x -> if x >= 3 then x - 1 else x + 1)

main :: IO ()
main = do
    input <- readFile "inputs/2017-05"
    let jumps = M.fromList $ zipWith (\x y -> (x, y)) [0..] (map (read :: String -> Int) (lines input))
    print $ solveOne jumps
    print $ solveTwo jumps
