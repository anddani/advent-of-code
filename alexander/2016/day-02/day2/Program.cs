using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace day2
{
    class Program
    {
        static void Main(string[] args)
        {

            //firstStar();
            secondStar();

            Console.WriteLine("done");
        }

        public static void firstStar()
        {
            string[] input = getInput();

            for (int i = 0; i < input.Length; i++)
            {
                string steps = input[i];

                int x = 0;
                int y = 0;

                foreach (char instruction in steps)
                {
                    switch (instruction)
                    {
                        case 'U':
                            y = modfiyCurrentKey(y, 1);
                            break;
                        case 'D':
                            y = modfiyCurrentKey(y, -1);
                            break;
                        case 'R':
                            x = modfiyCurrentKey(x, +1);
                            break;
                        case 'L':
                            x = modfiyCurrentKey(x, -1);
                            break;
                    }

                }

                Console.WriteLine("x: {0}, y: {1}", x, y);
            }

        }

        public static void secondStar()
        {
            const int keypadSize = 5;
            digit[][] keyPad = new digit[keypadSize][];

            for (int i = 0; i < keypadSize; i++)
            {
                keyPad[i] = new digit[keypadSize];

                for (int j = 0; j < keypadSize; j++)
                {
                    keyPad[i][j] = new digit(false);
                }
            }

            /*
             *  keyPad[row][Colum]; 
                  1
                2 3 4
              5 6 7 8 9
                A B C
                  D
           */

    
           keyPad[0][2].accessable = true;
           keyPad[0][2].key = '1';

           keyPad[1][1].accessable = true;
           keyPad[1][1].key = '2';

           keyPad[1][2].accessable = true;
           keyPad[1][2].key = '3';

           keyPad[1][3].accessable = true;
           keyPad[1][3].key = '4';

           keyPad[2][0].accessable = true;
           keyPad[2][0].key = '5';

           keyPad[2][1].accessable = true;
           keyPad[2][1].key = '6';

           keyPad[2][2].accessable = true;
           keyPad[2][2].key = '7';

           keyPad[2][3].accessable = true;
           keyPad[2][3].key = '8';

           keyPad[2][4].accessable = true;
           keyPad[2][4].key = '9';

           keyPad[3][1].accessable = true;
           keyPad[3][1].key = 'A';

           keyPad[3][2].accessable = true;
           keyPad[3][2].key = 'B';

           keyPad[3][3].accessable = true;
           keyPad[3][3].key = 'C';

           keyPad[4][2].accessable = true;
           keyPad[4][2].key = 'D';



            /*
             * for first star
            keyPad[0][0].accessable = true;
            keyPad[0][0].key = '1';

            keyPad[0][1].accessable = true;
            keyPad[0][1].key = '2';

            keyPad[0][2].accessable = true;
            keyPad[0][2].key = '3';

            keyPad[1][0].accessable = true;
            keyPad[1][0].key = '4';

            keyPad[1][1].accessable = true;
            keyPad[1][1].key = '5';

            keyPad[1][2].accessable = true;
            keyPad[1][2].key = '6';

            keyPad[2][0].accessable = true;
            keyPad[2][0].key = '7';

            keyPad[2][1].accessable = true;
            keyPad[2][1].key = '8';

            keyPad[2][2].accessable = true;
            keyPad[2][2].key = '9';
            */

            string[] input = getInput();

            int y = 2;
            int x = 0;

            for (int i = 0; i < input.Length; i++)
            {
                string steps = input[i];

                /* for first star
                int y = 1;
                int x = 1;
                */
                foreach (char instruction in steps)
                {
                    switch (instruction)
                    {
                        case 'U':
                            if (y > 0)
                            {
                                if (keyPad[y - 1][x].accessable)
                                {
                                    y = y -1;
                                }
                            }

                            break;
                        case 'D':
                            if (y < 4)
                            {
                                if (keyPad[y + 1][x].accessable)
                                {
                                    y = y+ 1;
                                }
                            }
                            break;

                        case 'R':

                            if (x < 4)
                            {
                                if (keyPad[y][x + 1].accessable)
                                {
                                    x = x+ 1;
                                }
                            }

                            break;
                        case 'L':
                            if (x > 0)
                            {
                                if (keyPad[y][x - 1].accessable)
                                {
                                    x = x- 1;
                                }
                            }
                            break;
                    }


                }

                Console.WriteLine("key: {0} , x: {1} y: {2}", keyPad[y][x].key, x, y);
            }

        }

        public static int modfiyCurrentKey(int x, int modifier)
        {
            if (x == 0)
            {
                x += modifier;
            }
            else if (x == -1)
            {
                if (modifier == 1)
                {
                    x += modifier;
                }
            }
            else if (x == 1)
            {
                if (modifier == -1)
                {
                    x += modifier;
                }
            }

            return x;
        }

        /* keypad 
         * 1 2 3
         * 4 5 6
         * 7 8 9 
         */

        public static string[] getInput()
        {
            string[] inputs = new string[5];
            //read 5 lines of input
            for (int i = 0; i < 5; i++)
            {
                inputs[i] = Console.ReadLine();
            }
            return inputs;
        }
    }


    public class digit
    {
        public bool accessable;
        public char key;

        public digit(bool v)
        {
            accessable = v;
        }
    }
}
