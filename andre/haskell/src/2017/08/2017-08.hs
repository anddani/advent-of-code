import Data.Either
import Data.List
import Data.Maybe
import Text.ParserCombinators.Parsec
import qualified Data.Map as M

data Condition = Condition
    { left :: String
    , ex :: Int -> Int -> Bool
    , right :: Int
    }

data Instruction = Instruction
    { var :: String
    , op :: Int -> Int -> Int
    , mod :: Int
    , cond :: Condition
    }

-- Parsing
parseNumber :: Parser Int
parseNumber = do
    neg <- (char '-' >> return "-") <|> return ""
    n <- many1 digit
    return $ read (neg ++ n)

parseConditionOp :: Parser (Int -> Int -> Bool)
parseConditionOp =
    try (string ">=" *> return (>=))
    <|> try (string "<=" *> return (<=))
    <|> try (string "==" *> return (==))
    <|> try (string "!=" *> return (/=))
    <|> try (string ">" *> return (>))
    <|> try (string "<" *> return (<))

parseModOp :: Parser (Int -> Int -> Int) 
parseModOp =
    string "inc" *> return (+)
    <|> string "dec" *> return (-)

parseCondition :: Parser Condition
parseCondition = do
    left <- space *> many1 letter
    ex <- space *> parseConditionOp
    right <- space *> parseNumber
    return $ Condition left ex right

parseStatement :: Parser Instruction
parseStatement = do
    var <- many1 letter
    op <- space *> parseModOp
    mod <- space *> parseNumber
    cond <- space *> string "if" *> parseCondition
    return $ Instruction var op mod cond

-- Part 1
getValue :: String -> M.Map String Int -> Int
getValue var registers =
    fromMaybe 0 $ M.lookup var registers

evaluateCondition :: Condition -> M.Map String Int -> Bool
evaluateCondition (Condition left ex right) registers =
    ex (getValue left registers) right

executeOperation :: M.Map String Int -> Instruction -> Int -> Int
executeOperation registers (Instruction _ op mod cond) currentValue =
    if evaluateCondition cond registers then
       op currentValue mod
    else currentValue

executeInstruction :: (M.Map String Int, Int) -> Instruction -> (M.Map String Int, Int)
executeInstruction (registers, largestFound) instruction@(Instruction var _ _ _) =
    (M.insert var newValue registers, max newValue largestFound)
        where newValue = executeOperation registers instruction (getValue var registers)

solve :: [String] -> (M.Map String Int, Int)
solve ins = foldl' executeInstruction (M.empty, 0) $ rights $ map (parse parseStatement "") ins

solveOne :: [String] -> Int
solveOne = maximum . M.elems . fst . solve

solveTwo :: [String] -> Int
solveTwo = snd . solve

main :: IO ()
main = do
    input <- readFile "inputs/2017-08"
    print $ solveOne (lines input)
    print $ solveTwo (lines input)
