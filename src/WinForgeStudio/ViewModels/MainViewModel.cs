using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Windows;
using WinForgeStudio.Models;
using WinForgeStudio.Services;

namespace WinForgeStudio.ViewModels;

public sealed class MainViewModel : INotifyPropertyChanged
{
    private readonly SystemInspector _inspector = new();
    private readonly ChangeJournal _journal = new();
    private readonly ServiceManager _services;
    private string _status = "Ready";
    private string _search = "";
    private string _language = "EN";
    private string _processSearch = "";
    private string _startupSearch = "";

    public event PropertyChangedEventHandler? PropertyChanged;
    public ObservableCollection<ServiceInfo> Services { get; } = new();
    public ObservableCollection<ProcessInfo> Processes { get; } = new();
    public IEnumerable<ProcessInfo> FilteredProcesses => Processes.Where(p => string.IsNullOrWhiteSpace(ProcessSearch) || p.Name.Contains(ProcessSearch, StringComparison.OrdinalIgnoreCase) || p.Id.ToString().Contains(ProcessSearch, StringComparison.OrdinalIgnoreCase) || (p.FilePath?.Contains(ProcessSearch, StringComparison.OrdinalIgnoreCase) ?? false));
    public ObservableCollection<StartupInfo> Startup { get; } = new();
    public IEnumerable<StartupInfo> FilteredStartup => Startup.Where(s => string.IsNullOrWhiteSpace(StartupSearch) || s.Name.Contains(StartupSearch, StringComparison.OrdinalIgnoreCase) || s.Command.Contains(StartupSearch, StringComparison.OrdinalIgnoreCase) || s.Location.Contains(StartupSearch, StringComparison.OrdinalIgnoreCase));
    public SystemSnapshot? Snapshot { get; private set; }

    public string Status { get => _status; set { _status = value; OnChanged(); } }
    public string Search { get => _search; set { _search = value; OnChanged(); OnChanged(nameof(FilteredServices)); } }
    public string ProcessSearch { get => _processSearch; set { _processSearch = value; OnChanged(); OnChanged(nameof(FilteredProcesses)); } }
    public string StartupSearch { get => _startupSearch; set { _startupSearch = value; OnChanged(); OnChanged(nameof(FilteredStartup)); } }
    public string Language { get => _language; set { _language = value; OnChanged(); OnChanged(nameof(IsPersian)); } }
    public bool IsPersian => Language == "FA";
    public string JournalPath => _journal.JournalPath;

    public IEnumerable<ServiceInfo> FilteredServices => Services.Where(s =>
        string.IsNullOrWhiteSpace(Search) ||
        s.Name.Contains(Search, StringComparison.OrdinalIgnoreCase) ||
        s.DisplayName.Contains(Search, StringComparison.OrdinalIgnoreCase));

    public MainViewModel() => _services = new ServiceManager(_journal);

    public void Refresh()
    {
        try
        {
            Snapshot = _inspector.GetSnapshot();
            OnChanged(nameof(Snapshot));
            Services.Clear();
            foreach (var service in _services.GetServices()) Services.Add(service);
            Processes.Clear();
            foreach (var process in _inspector.GetProcesses().Take(500)) Processes.Add(process);
            OnChanged(nameof(FilteredProcesses));
            Startup.Clear();
            foreach (var item in _inspector.GetStartupEntries()) Startup.Add(item);
            OnChanged(nameof(FilteredStartup));
            Status = "Scan completed. No changes were made.";
        }
        catch (Exception ex) { Status = ex.Message; }
    }

    public void ChangeStartup(ServiceInfo? item, string type)
    {
        if (item is null) return;
        var result = MessageBox.Show(
            $"Service: {item.DisplayName}\nCurrent: {item.StartType}\nNew: {type}\n\nRisk: {item.Risk}\n{item.RiskReason}\n\nContinue?",
            "Confirm service change", MessageBoxButton.YesNo, MessageBoxImage.Warning);
        if (result != MessageBoxResult.Yes) return;
        try { _services.SetStartupType(item.Name, type); Refresh(); Status = $"Updated {item.Name}"; }
        catch (Exception ex) { Status = ex.Message; }
    }


    public void EndProcess(ProcessInfo? item)
    {
        if (item is null) { Status = "Select a process first."; return; }
        var protectedNames = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "System", "Idle", "Registry", "smss", "csrss", "wininit", "services", "lsass", "winlogon", "explorer" };
        if (protectedNames.Contains(item.Name)) { Status = "WinForge blocked termination of a protected Windows process."; return; }
        var confirm = MessageBox.Show($"End process {item.Name} (PID {item.Id})?\n\nThis may close the associated application and lose unsaved work.", "Confirm process termination", MessageBoxButton.YesNo, MessageBoxImage.Warning);
        if (confirm != MessageBoxResult.Yes) return;
        try
        {
            using var process = System.Diagnostics.Process.GetProcessById(item.Id);
            if (!string.Equals(process.ProcessName, item.Name, StringComparison.OrdinalIgnoreCase)) throw new InvalidOperationException("PID identity changed; action cancelled.");
            process.Kill(entireProcessTree: false);
            process.WaitForExit(3000);
            Status = $"Termination requested for {item.Name} (PID {item.Id}).";
            Refresh();
        }
        catch (Exception ex) { Status = $"Could not end process: {ex.Message}"; }
    }

    public void ExportReport()
    {
        var dialog = new Microsoft.Win32.SaveFileDialog { Title = "Export WinForge system report", Filter = "Text report (*.txt)|*.txt", FileName = $"WinForge-Report-{DateTime.Now:yyyyMMdd-HHmm}.txt" };
        if (dialog.ShowDialog() != true) return;
        try
        {
            var snapshot = Snapshot;
            var lines = new List<string> { "WinForge Studio System Report", $"Generated: {DateTime.Now:yyyy-MM-dd HH:mm:ss}", "READ-ONLY INVENTORY", "" };
            if (snapshot is not null) lines.AddRange(new[] { $"Computer: {snapshot.ComputerName}", $"Windows: {snapshot.WindowsCaption} {snapshot.Version} (build {snapshot.Build})", $"Architecture: {snapshot.Architecture}", $"CPU: {snapshot.CpuName}", $"Logical processors: {snapshot.LogicalProcessors}", $"RAM: {snapshot.TotalRamGb:N2} GB", $"System drive: {snapshot.SystemDrive} | Free: {snapshot.DriveFreeGb:N2} GB / {snapshot.DriveTotalGb:N2} GB", $"Administrator: {snapshot.IsAdministrator}", "" });
            lines.Add("Processes (top memory)"); lines.AddRange(Processes.Take(500).Select(p => $"{p.Name} | PID {p.Id} | {p.MemoryMb:N1} MB | {p.FilePath}")); lines.Add("");
            lines.Add("Services"); lines.AddRange(Services.Select(s => $"{s.DisplayName} [{s.Name}] | {s.Status} | Startup: {s.StartType} | Risk: {s.Risk}")); lines.Add("");
            lines.Add("Startup entries"); lines.AddRange(Startup.Select(s => $"{s.Name} | {s.Location} | {s.Command}")); 
            System.IO.File.WriteAllText(dialog.FileName, string.Join(Environment.NewLine, lines), new System.Text.UTF8Encoding(true));
            Status = $"Report exported: {dialog.FileName}";
        }
        catch (Exception ex) { Status = $"Report export failed: {ex.Message}"; }
    }

    private void OnChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
