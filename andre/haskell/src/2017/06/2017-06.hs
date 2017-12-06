import Data.List
import Data.Ord
import Data.Maybe(fromJust)
import qualified Data.Set as S

data BlockState = BlockState (S.Set [Int]) [Int]

maxBy :: [(Int, Int)] -> (Int, Int)
maxBy = foldl' (\acc@(_, maxFound) curr@(_, x) -> if x > maxFound then curr else acc) (-1, -1)

setZero :: [Int] -> Int -> [Int]
setZero xs n = take n xs ++ [0] ++ drop (n + 1) xs

transform :: [Int] -> [Int]
transform blocks = 
    let (index, maxVal) = maxBy $ zip [0..] blocks
        numBlocks = length blocks
        increase = maxVal `quot` numBlocks
        rest = maxVal `mod` numBlocks
        increasedList = map (+ increase) (setZero blocks index)
        indicesToUpdate = map (`mod` numBlocks) [index+1..index+rest]
        result = map (\(x, y) -> if x `elem` indicesToUpdate then y + 1 else y) $ zip [0..] increasedList
     in result

realloc :: BlockState -> Maybe ([Int], BlockState)
realloc (BlockState set curr)
  | S.member curr set = Nothing
  | otherwise = Just (nextState, BlockState (S.insert curr set) nextState)
  where nextState = transform curr

solveOne :: BlockState -> [[Int]]
solveOne = unfoldr realloc

main :: IO ()
main = do
    input <- readFile "inputs/2017-06"
    let banks = map (read :: String -> Int) $ words input
        solution = solveOne $ BlockState S.empty banks
        part1 = length solution
        part2 = part1 - 1 - fromJust (elemIndex (last solution) solution)
    print part1
    print part2
