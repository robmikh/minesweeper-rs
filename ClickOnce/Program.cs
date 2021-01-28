using System;
using System.Diagnostics;
using System.Windows.Forms;

namespace WindowsFormsApp1
{
    static class Program
    {
        static void Main()
        {
            try
            {
                Process.Start(new ProcessStartInfo { CreateNoWindow = true, FileName = "minesweeper-rs.exe" });
            }
            catch (Exception x)
            {
                MessageBox.Show(x.Message, "Error Starting Minesweeper", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }
    }
}
