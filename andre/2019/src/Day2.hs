module Day2 where

import Control.Lens
import Data.List (unfoldr)
import Data.List.Split
import qualified Data.Sequence as S

type Op = Int -> Int -> Int

opValue :: S.Seq Int -> Int -> Int
opValue intCode index = S.index intCode (S.index intCode index)

applyOperation :: S.Seq Int -> Op -> Int -> Int -> Int -> S.Seq Int
applyOperation intCode operation i1 i2 d =
  S.update d (operation i1 i2) intCode

applyOpCode :: (S.Seq Int, Int) -> Maybe (S.Seq Int)
applyOpCode (intCode, pc)
  | opCode == 1 = Just $ applyOperation intCode (+) i1 i2 d
  | opCode == 2 = Just $ applyOperation intCode (*) i1 i2 d
  | opCode == 99 = Nothing
  where opCode = S.index intCode pc
        i1 = opValue intCode $ pc + 1
        i2 = opValue intCode $ pc + 2
        d = S.index intCode $ pc + 3

nextOperation :: Int -> S.Seq Int -> (S.Seq Int, (S.Seq Int, Int))
nextOperation pc seq = (seq, (seq, pc + 4))

part1 :: S.Seq Int -> Int
part1 intCode =
  let lastState :: S.Seq Int
      lastState = last $ unfoldr (\(newIntCode, pc) -> nextOperation pc <$> applyOpCode (newIntCode, pc)) (intCode, 0)
  in S.index lastState 0

solve1 :: IO ()
solve1 = do
  file <- readFile "./input/d2p1.input"
  let input = map (\x -> read x :: Int) $ splitOn "," file
  let list = S.fromList input
  print $ part1 list

