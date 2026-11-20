using System.Diagnostics;
using System.Management;
using System.Security.Principal;
using WinForgeStudio.Models;

namespace WinForgeStudio.Services;

public sealed class SystemInspector
{
    public SystemSnapshot GetSnapshot()
    {
        using var os = new ManagementObjectSearcher("SELECT Caption, Version, BuildNumber FROM Win32_OperatingSystem");
        using var cpu = new ManagementObjectSearcher("SELECT Name, NumberOfLogicalProcessors FROM Win32_Processor");
        using var computer = new ManagementObjectSearcher("SELECT TotalPhysicalMemory FROM Win32_ComputerSystem");
        using var osRows = os.Get();
        using var cpuRows = cpu.Get();
        using var memoryRows = computer.Get();
        using var o = osRows.Cast<ManagementObject>().FirstOrDefault();
        using var c = cpuRows.Cast<ManagementObject>().FirstOrDefault();
        using var m = memoryRows.Cast<ManagementObject>().FirstOrDefault();

        var drive = new DriveInfo(Environment.GetFolderPath(Environment.SpecialFolder.System)[..2]);
        var total = Convert.ToDouble(m?["TotalPhysicalMemory"] ?? 0UL) / 1024 / 1024 / 1024;
        var available = GC.GetGCMemoryInfo().TotalAvailableMemoryBytes / 1024d / 1024 / 1024;

        return new SystemSnapshot(
            Environment.MachineName, S(o?["Caption"]), S(o?["Version"]), S(o?["BuildNumber"]),
            Environment.Is64BitOperatingSystem ? "x64" : "x86", S(c?["Name"]),
            Convert.ToInt32(c?["NumberOfLogicalProcessors"] ?? 0), total,
            Math.Min(available, total), drive.Name, drive.AvailableFreeSpace / 1024d / 1024 / 1024,
            drive.TotalSize / 1024d / 1024 / 1024, IsAdmin());
    }

    public IReadOnlyList<ProcessInfo> GetProcesses()
    {
        var list = new List<ProcessInfo>();
        foreach (var p in Process.GetProcesses())
        {
            try
            {
                list.Add(new(p.Id, p.ProcessName, SafePath(p), p.WorkingSet64 / 1024d / 1024,
                    p.TotalProcessorTime.TotalSeconds));
            }
            catch { /* Protected or exited processes may deny access. */ }
            finally { p.Dispose(); }
        }
        return list.OrderByDescending(x => x.MemoryMb).ToList();
    }

    public IReadOnlyList<StartupInfo> GetStartupEntries()
    {
        var results = new List<StartupInfo>();
        ReadRunKey(Microsoft.Win32.Registry.CurrentUser,
            @"Software\Microsoft\Windows\CurrentVersion\Run", "HKCU Run", results);
        ReadRunKey(Microsoft.Win32.Registry.LocalMachine,
            @"Software\Microsoft\Windows\CurrentVersion\Run", "HKLM Run", results);

        var startup = Environment.GetFolderPath(Environment.SpecialFolder.Startup);
        if (Directory.Exists(startup))
            foreach (var file in Directory.GetFiles(startup))
                results.Add(new(Path.GetFileName(file), file, "User Startup folder", "Review"));
        return results;
    }

    private static void ReadRunKey(Microsoft.Win32.RegistryKey root, string path, string label, List<StartupInfo> output)
    {
        try
        {
            using var key = root.OpenSubKey(path);
            if (key is null) return;
            foreach (var name in key.GetValueNames())
                output.Add(new(name, Convert.ToString(key.GetValue(name)) ?? "", label, "Review"));
        }
        catch { }
    }

    private static string? SafePath(Process process)
    {
        try { return process.MainModule?.FileName; }
        catch { return null; }
    }

    private static string S(object? value) => Convert.ToString(value) ?? "Unknown";

    private static bool IsAdmin()
    {
        using var identity = WindowsIdentity.GetCurrent();
        return new WindowsPrincipal(identity).IsInRole(WindowsBuiltInRole.Administrator);
    }
}
