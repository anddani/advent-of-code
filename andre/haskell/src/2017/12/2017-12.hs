import Data.Either
import qualified Data.IntMap as M
import Data.List
import Data.Ord
import Data.Maybe (fromJust)
import qualified Data.IntSet as S
import Control.Arrow (second)
import Text.ParserCombinators.Parsec

type Program = (Int, [Int])
type Pipe = S.IntSet
type PipeMap = M.IntMap Pipe

-- Parse
parsePipe :: Parser Program
parsePipe = do
    x <- many1 digit <* string " <-> "
    xs <- many1 digit `sepBy` string ", "
    return (read x, map read xs)

parsePipes :: [String] -> [Program]
parsePipes ins = rights $ map (parse parsePipe "") ins

-- Problem
toPipeMap :: [Program] -> PipeMap
toPipeMap = M.fromList . map (second S.fromList)

getPipeContent :: PipeMap -> Int -> Pipe
getPipeContent pipes head =
    M.findWithDefault S.empty head pipes

mergePipe :: PipeMap -> Pipe -> Int -> Pipe
mergePipe pipes path head =
    let visited = S.insert head path
        children = S.difference (getPipeContent pipes head) visited
        others = S.unions . map (mergePipe pipes visited) . S.toList $ children
     in S.unions [S.singleton head, children, others]

mergeAllPipes :: PipeMap -> PipeMap
mergeAllPipes pipes =
    foldl' (\acc key -> 
        case M.lookup key acc of
          Just children ->
              let newPipe = mergePipe acc S.empty key
                  newPipeMap = M.withoutKeys acc (S.fromList (delete key (S.elems newPipe)))
               in M.insert key newPipe newPipeMap
          Nothing -> acc
        ) pipes (M.keys pipes)

solveOne :: PipeMap -> Int
solveOne = S.size . fromJust . find (S.member 0) . M.elems

solveTwo :: PipeMap -> Int
solveTwo = length

main :: IO ()
main = do
    input <- readFile "inputs/2017-12"
    let pipes = mergeAllPipes . toPipeMap $ parsePipes (lines input)
    print $ solveOne pipes
    print $ solveTwo pipes
