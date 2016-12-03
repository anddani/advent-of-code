using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace day3
{
    class Program
    {
        static void Main(string[] args)
        {

            //partOne();
            partTwo();
        }

        public static void partOne()
        {

            List<string> input = getInput();

            int count = 0;

            foreach (string triangles in input)
            {

                string[] split = triangles.Substring(2).Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
                int x = int.Parse(split[0]);
                int y = int.Parse(split[1]);
                int z = int.Parse(split[2]);

                if ((x + y) > z && (x + z) > y && (y + z) > x)
                {
                    count++;

                }
            }

            Console.WriteLine(count);


        }

        public static void partTwo()
        {

            List<string> input = getInput();

            int count = 0;

            for (int i = 0; i < input.Count; i = i + 3)
            {

                string[] one = input[i].Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
                string[] two = input[i + 1].Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
                string[] three = input[i + 2].Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
                int columnIndex = 0;

                if (checkTriangle(int.Parse(one[columnIndex]), int.Parse(two[columnIndex]), int.Parse(three[columnIndex])))
                {
                    count++;
                }

                columnIndex++;

                if (checkTriangle(int.Parse(one[columnIndex]), int.Parse(two[columnIndex]), int.Parse(three[columnIndex])))
                {
                    count++;
                }
                columnIndex++;

                if (checkTriangle(int.Parse(one[columnIndex]), int.Parse(two[columnIndex]), int.Parse(three[columnIndex])))
                {
                    count++;
                }

            }

            Console.WriteLine(count);
        }

        public static bool checkTriangle(int x , int y , int z)
        {

            if ((x + y) > z && (x + z) > y && (y + z) > x)
            {
                return true;

            }
            return false;
        }
        public static List<string> getInput()
        {
            List<string> inputs = new List<string>();

            string s;
            //read 5 lines of input
            while ((s = Console.ReadLine()) != null)
            {
                inputs.Add(s);
            }
            return inputs;
        }
    }
}
