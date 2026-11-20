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

    public event PropertyChangedEventHandler? PropertyChanged;
    public ObservableCollection<ServiceInfo> Services { get; } = new();
    public ObservableCollection<ProcessInfo> Processes { get; } = new();
    public ObservableCollection<StartupInfo> Startup { get; } = new();
    public SystemSnapshot? Snapshot { get; private set; }

    public string Status { get => _status; set { _status = value; OnChanged(); } }
    public string Search { get => _search; set { _search = value; OnChanged(); OnChanged(nameof(FilteredServices)); } }
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
            foreach (var process in _inspector.GetProcesses().Take(250)) Processes.Add(process);
            Startup.Clear();
            foreach (var item in _inspector.GetStartupEntries()) Startup.Add(item);
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

    private void OnChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
