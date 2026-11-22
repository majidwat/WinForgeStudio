using System.Management;
using System.ServiceProcess;
using Microsoft.Win32;
using WinForgeStudio.Models;

namespace WinForgeStudio.Services;

public sealed class ServiceManager
{
    private readonly ChangeJournal _journal;
    public ServiceManager(ChangeJournal journal) => _journal = journal;

    public IReadOnlyList<ServiceInfo> GetServices()
    {
        var rows = new List<ServiceInfo>();
        using var searcher = new ManagementObjectSearcher(
            "SELECT Name, DisplayName, State, StartMode, StartName, Description, PathName FROM Win32_Service");
        using var collection = searcher.Get();

        foreach (ManagementObject item in collection)
        {
            using (item)
            {
                var name = Convert.ToString(item["Name"]) ?? "";
                var display = Convert.ToString(item["DisplayName"]) ?? name;
                var path = Convert.ToString(item["PathName"]) ?? "";
                var rawStart = Convert.ToString(item["StartMode"]) ?? "Unknown";
                var start = rawStart.Equals("Auto", StringComparison.OrdinalIgnoreCase) ? "Automatic" : rawStart;
                var (risk, reason) = Assess(name, display, path, start);
                rows.Add(new(name, display, Convert.ToString(item["State"]) ?? "Unknown",
                    start, Convert.ToString(item["StartName"]) ?? "",
                    Convert.ToString(item["Description"]) ?? "", path, risk, reason));
            }
        }
        return rows.OrderBy(x => x.DisplayName).ToList();
    }

    public void SetStartupType(string serviceName, string startType)
    {
        if (startType is not ("Automatic" or "Manual" or "Disabled"))
            throw new ArgumentException("Unsupported startup type.");

        var service = GetServices().FirstOrDefault(s => s.Name == serviceName)
            ?? throw new InvalidOperationException("Service not found.");

        if (startType == "Disabled" && service.Risk != "Low")
            throw new InvalidOperationException("WinForge only permits disabling services classified Low risk; this service is not.");

        using var key = Registry.LocalMachine.OpenSubKey(
            $@"SYSTEM\CurrentControlSet\Services\{serviceName}", writable: true)
            ?? throw new InvalidOperationException("Service registry key unavailable. Run as Administrator.");

        var oldValue = key.GetValue("Start");
        var oldKind = key.GetValueKind("Start");
        var oldType = Convert.ToString(oldValue) ?? "Unknown";
        var newValue = startType switch { "Automatic" => 2, "Manual" => 3, _ => 4 };

        // If the journal cannot be written, do not mutate the machine.
        _journal.Record("ServiceStartup", serviceName, $"{oldType} ({oldKind})",
            $"{newValue} (DWord)", $"Requested {startType} for {service.DisplayName}");
        key.SetValue("Start", newValue, RegistryValueKind.DWord);
        var verified = Convert.ToInt32(key.GetValue("Start"));
        if (verified != newValue)
            throw new InvalidOperationException("The service startup value did not verify after writing.");
    }

    public void Start(string name) => WithService(name, service => { service.Start(); service.WaitForStatus(ServiceControllerStatus.Running, TimeSpan.FromSeconds(15)); });
    public void Stop(string name) => WithService(name, service => { service.Stop(); service.WaitForStatus(ServiceControllerStatus.Stopped, TimeSpan.FromSeconds(15)); });

    private static void WithService(string name, Action<ServiceController> action)
    {
        using var controller = new ServiceController(name);
        action(controller);
    }

    private static (string Risk, string Reason) Assess(string name, string display, string path, string start)
    {
        var n = name.ToLowerInvariant();
        if (n is "rpcss" or "dcomlaunch" or "eventlog" or "plugplay" or "power" or "lsm" or "samss")
            return ("Critical", "Core Windows infrastructure; do not disable.");
        if (n.Contains("defend") || n.Contains("firewall") || n.Contains("wuauserv") ||
            n.Contains("cryptsvc") || n.Contains("bits") || n.Contains("winmgmt"))
            return ("High", "Security, servicing, or management component. Do not disable as a generic optimization.");
        return ("Unknown", "No safe-disable conclusion is available. Review publisher, dependencies, and application impact.");
    }
}
