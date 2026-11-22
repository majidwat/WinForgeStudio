using System.Windows;
using System.Windows.Controls;
using WinForgeStudio.Models;
using WinForgeStudio.ViewModels;

namespace WinForgeStudio;

public partial class MainWindow : Window
{
    private readonly MainViewModel _vm = new();

    public MainWindow()
    {
        InitializeComponent();
        ServicesGrid.ItemsSource = _vm.FilteredServices;
        ProcessesGrid.ItemsSource = _vm.FilteredProcesses;
        StartupGrid.ItemsSource = _vm.FilteredStartup;
        JournalPathText.Text = _vm.JournalPath;

        Loaded += (_, _) => { _ = _vm.RefreshAsync(); };
        _vm.PropertyChanged += (_, e) =>
        {
            if (e.PropertyName == nameof(MainViewModel.Status)) StatusText.Text = _vm.Status;
            if (e.PropertyName == nameof(MainViewModel.Snapshot)) UpdateOverview();
            if (e.PropertyName == nameof(MainViewModel.FilteredServices)) ServicesGrid.ItemsSource = _vm.FilteredServices;
            if (e.PropertyName == nameof(MainViewModel.FilteredProcesses)) ProcessesGrid.ItemsSource = _vm.FilteredProcesses;
            if (e.PropertyName == nameof(MainViewModel.FilteredStartup)) StartupGrid.ItemsSource = _vm.FilteredStartup;
        };
    }

    private void Show(string panel, string title)
    {
        OverviewPanel.Visibility = ServicesPanel.Visibility = ProcessesPanel.Visibility =
            StartupPanel.Visibility = JournalPanel.Visibility = SettingsPanel.Visibility = Visibility.Collapsed;
        switch (panel)
        {
            case "overview": OverviewPanel.Visibility = Visibility.Visible; UpdateOverview(); break;
            case "services": ServicesPanel.Visibility = Visibility.Visible; break;
            case "processes": ProcessesPanel.Visibility = Visibility.Visible; break;
            case "startup": StartupPanel.Visibility = Visibility.Visible; break;
            case "journal": JournalPanel.Visibility = Visibility.Visible; break;
            case "settings": SettingsPanel.Visibility = Visibility.Visible; break;
        }
        PageTitle.Text = title;
    }

    private void UpdateOverview()
    {
        if (_vm.Snapshot is not { } s) return;
        WindowsValue.Text = $"{s.WindowsCaption} • {s.Build}";
        RamValue.Text = $"{s.TotalRamGb:N1} GB";
        CpuValue.Text = s.LogicalProcessors.ToString();
        DiskValue.Text = $"{s.DriveFreeGb:N1} GB";
        DetailsValue.Text =
            $"Computer: {s.ComputerName}\nWindows: {s.WindowsCaption}\nVersion / Build: {s.Version} / {s.Build}\n" +
            $"Architecture: {s.Architecture}\nCPU: {s.CpuName}\nLogical processors: {s.LogicalProcessors}\n" +
            $"RAM: {s.TotalRamGb:N2} GB installed\nSystem drive: {s.SystemDrive} | {s.DriveFreeGb:N1} GB free of {s.DriveTotalGb:N1} GB\n" +
            $"Administrator: {(s.IsAdministrator ? "Yes" : "No")}";
    }

    private void Refresh_Click(object sender, RoutedEventArgs e)
    {
        _ = _vm.RefreshAsync();
    }

    private void Overview_Click(object sender, RoutedEventArgs e) => Show("overview", "System Overview");
    private void Services_Click(object sender, RoutedEventArgs e) => Show("services", "Service Manager");
    private void Processes_Click(object sender, RoutedEventArgs e) => Show("processes", "Process Explorer");
    private void Startup_Click(object sender, RoutedEventArgs e) => Show("startup", "Startup Inventory");
    private void Journal_Click(object sender, RoutedEventArgs e) => Show("journal", "Change Journal");
    private void Settings_Click(object sender, RoutedEventArgs e) => Show("settings", "Settings");

    private void Search_Changed(object sender, TextChangedEventArgs e)
    {
        if (!IsInitialized) return;
        _vm.Search = ServiceSearch.Text;
        ServicesGrid.ItemsSource = _vm.FilteredServices;
    }

    private void StartService_Click(object sender, RoutedEventArgs e) => _vm.StartService(ServicesGrid.SelectedItem as ServiceInfo);
    private void StopService_Click(object sender, RoutedEventArgs e) => _vm.StopService(ServicesGrid.SelectedItem as ServiceInfo);
    private void Manual_Click(object sender, RoutedEventArgs e) => _vm.ChangeStartup(ServicesGrid.SelectedItem as ServiceInfo, "Manual");
    private void Automatic_Click(object sender, RoutedEventArgs e) => _vm.ChangeStartup(ServicesGrid.SelectedItem as ServiceInfo, "Automatic");
    private void Disable_Click(object sender, RoutedEventArgs e) => _vm.ChangeStartup(ServicesGrid.SelectedItem as ServiceInfo, "Disabled");


    private void ProcessSearch_Changed(object sender, TextChangedEventArgs e)
    {
        if (!IsInitialized) return;
        _vm.ProcessSearch = ProcessSearch.Text;
        ProcessesGrid.ItemsSource = _vm.FilteredProcesses;
    }
    private void StartupSearch_Changed(object sender, TextChangedEventArgs e)
    {
        if (!IsInitialized) return;
        _vm.StartupSearch = StartupSearch.Text;
        StartupGrid.ItemsSource = _vm.FilteredStartup;
    }
    private void EndProcess_Click(object sender, RoutedEventArgs e) => _vm.EndProcess(ProcessesGrid.SelectedItem as ProcessInfo);
    private void Export_Click(object sender, RoutedEventArgs e) => _vm.ExportReport();

    private void Language_Changed(object sender, SelectionChangedEventArgs e)
    {
        // Placeholder until full resource-based EN/FA and RTL localization is implemented.
    }
}
