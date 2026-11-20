using System.Text.Json;

namespace WinForgeStudio.Services;

public sealed class ChangeJournal
{
    private readonly string _path = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
        "WinForgeStudio", "change-journal.jsonl");

    public void Record(string category, string target, string before, string after, string note)
    {
        Directory.CreateDirectory(Path.GetDirectoryName(_path)!);
        var line = JsonSerializer.Serialize(new {
            TimestampUtc = DateTimeOffset.UtcNow, Category = category,
            Target = target, Before = before, After = after, Note = note
        });
        File.AppendAllText(_path, line + Environment.NewLine);
    }

    public string JournalPath => _path;
}
