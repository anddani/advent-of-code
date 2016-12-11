using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace day4
{
    class Program
    {
        static void Main(string[] args)
        {
            List<string[]> inputs = getInput();
            for (int i = 0; i < inputs.Count; i++)
            {

                List<container> tracking = new List<container>();

                for (int j = 0; j < inputs[i].Length - 1; j++)
                {
                    foreach (char x in inputs[i][j])
                    {
                        updateTrackingdata(tracking, x);
                    }
                }

            }
        }

        public static void updateTrackingdata(List<container> derp, char x)
        {

            for (int k = 0; k < derp.Count; k++)
            {
                if (derp[k].letter == x)
                {
                    derp[k].count += 1;
                    return;
                }
            }

            derp.Add(new container(x));
            return;
        }


        public static List<string[]> getInput()
        {
            List<string[]> inputs = new List<string[]>();

            string s;
            //read 5 lines of input
            while ((s = Console.ReadLine()) != null)
            {
                inputs.Add(s.Split('-'));
            }

            return inputs;
        }

    }

    public class container
    {

        public int count = 0;
        public char letter;

        public container(char x)
        {
            letter = x;
            count = 1;
        }
    }

}
