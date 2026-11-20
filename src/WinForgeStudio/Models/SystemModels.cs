namespace WinForgeStudio.Models;

public sealed record ServiceInfo(
    string Name, string DisplayName, string Status, string StartType,
    string Account, string Description, string? ImagePath, string Risk, string RiskReason);

public sealed record ProcessInfo(
    int Id, string Name, string? FilePath, double MemoryMb, double CpuSeconds);

public sealed record StartupInfo(
    string Name, string Command, string Location, string Risk);

public sealed record SystemSnapshot(
    string ComputerName, string WindowsCaption, string Version, string Build,
    string Architecture, string CpuName, int LogicalProcessors,
    double TotalRamGb, double AvailableRamGb, string SystemDrive,
    double DriveFreeGb, double DriveTotalGb, bool IsAdministrator);
