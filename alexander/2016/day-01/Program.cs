using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Collections;

namespace Day_1_No_Time_for_a_Taxicab
{
    class Program
    {
        public static int x = 0;
        public static int y = 0;

        public static List<int> PreviousLocationsx = new List<int>();
        public static List<int> PreviousLocationsy = new List<int>();

        static void Main(string[] args)
        {

            Direction dir = Direction.NORTH;
            string[] instructions = getInput();

            foreach (string instruction in instructions)
            {

                int numOfSteps = int.Parse(instruction.Substring(1));
                char turn = instruction.ToArray()[0];

                dir = setNewDircetion(dir, turn);

                switch (dir)
                {
                    case Direction.NORTH:

                        for (int i = 0; i < numOfSteps; i++)
                        {
                            y += 1;
                            didweWin();
                            PreviousLocationsx.Add(x);
                            PreviousLocationsy.Add(y);
                        }

                        break;
                    case Direction.SOUTH:

                        for (int i = 0; i < numOfSteps; i++)
                        {
                            y -= 1;
                            didweWin();
                            PreviousLocationsx.Add(x);
                            PreviousLocationsy.Add(y);
                        }

                        break;
                    case Direction.EAST:

                        for (int i = 0; i < numOfSteps; i++)
                        {
                            x += 1;
                            didweWin();
                            PreviousLocationsx.Add(x);
                            PreviousLocationsy.Add(y);
                        }
                        break;
                    case Direction.WEST:

                        for (int i = 0; i < numOfSteps; i++)
                        {
                            x -= 1;
                            didweWin();
                            PreviousLocationsx.Add(x);
                            PreviousLocationsy.Add(y);
                        }
                        break;
                }

            }
        }

        public static void didweWin()
        {

            for (int i = 0; i < PreviousLocationsx.Count; i++)
            {
                if (PreviousLocationsy[i] == y && PreviousLocationsx[i] == x)
                {
                    int answer2 = Math.Abs(x) + Math.Abs(y);
                    Console.WriteLine("Answer is: {0}", answer2);
                    System.Environment.Exit(1);
                    return;
                }
            }
        }


        public enum Direction { NORTH, SOUTH, EAST, WEST };

        public static string[] getInput()
        {
            string input = Console.ReadLine();

            string[] instructions = input.Replace(" ", string.Empty).Split(',');

            return instructions;
        }

        public static Direction setNewDircetion(Direction direction, char turn)
        {

            switch (direction)
            {
                case Direction.NORTH:

                    if (turn == 'R')
                    {
                        return Direction.EAST;
                    }
                    else
                    {
                        return Direction.WEST;
                    }

                case Direction.SOUTH:
                    if (turn == 'R')
                    {
                        return Direction.WEST;
                    }
                    else
                    {
                        return Direction.EAST;
                    }

                case Direction.EAST:
                    if (turn == 'R')
                    {
                        return Direction.SOUTH;
                    }
                    else
                    {
                        return Direction.NORTH;
                    }

                case Direction.WEST:
                    if (turn == 'R')
                    {
                        return Direction.NORTH;
                    }
                    else
                    {
                        return Direction.SOUTH;
                    }
            }
            return direction;
        }
    }

}
