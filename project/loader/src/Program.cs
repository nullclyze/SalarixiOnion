using System;
using System.IO;
using System.Threading;
using System.Diagnostics;
using System.IO.Pipes;

class Loader
{
  static void Main()
  {
    Console.WriteLine("\n\t------------| Загрузчик Salarixi Onion |------------\n");

    string[] services = new[]
    {
      "so-utils",
      "so-core",
      "so-interface"
    };

    foreach (string service in services)
    {
      KillOldProcess(service);
    }

    Thread.Sleep(2000);

    string[] possiblePaths = new[]
    {
      "./services/@.exe",
      "../services/@.exe",
      "./@.exe",
      "../@.exe",
      "../../services/@.exe",
      "../../@.exe",
      "../../@.exe",
      "../../../@.exe",
      "../../../services/@.exe"
    };

    foreach (string service in services)
    {
      bool status = false;

      Write("Launch", $"Запуск сервиса {service}...");

      foreach (string possiblePath in possiblePaths)
      {
        if (status) break;

        string path = possiblePath.Replace("@", service);

        Write("Launch", $"Пробуем путь {path}...");

        bool operation = StartProcess(path, service);

        if (operation)
        {
          Write("Launch", $"Путь {path} ==> Валидный");
          status = true;
        } else
        {
          Write("Launch", $"Путь {path} ==> Не валидный");
        }
      }

      if (!status)
      {
        Write("Launch ~ Error", $"Не удалось запустить сервис {service}");
      } else
      {
        Write("Launch ~ Success", $"Сервис {service} успешно запущен");
      }
    }
  }

  static void Write(string prefix, string text)
  {
    Console.WriteLine($"INFO / {prefix}: {text}");
  }

  static string GetFullPath(string path)
  {
    var fullPath = Path.GetFullPath(path);

    if (!File.Exists(fullPath))
    {
      return "";
    }
    else
    {
      return fullPath;
    }
  }
  
  static void KillOldProcess(string name)
  {
    if (Process.GetProcessesByName(name).Length > 0)
    {
      foreach (var process in Process.GetProcessesByName(name))
      {
        try
        {
          process.Kill();
          process.WaitForExit(3000); 
          Write("Kill Process", $"Сервис {name} успешно остановлен (PID: {process.Id})");
        }
        catch (Exception ex)
        {
          Write("Kill Process", $"Не удалось остановить сервис {name} (PID: {process.Id}): {ex.Message}");
        }
      }
    } else
    {
      Write("Kill Process", $"Сервис {name} неактивен, остановка невозможна");
    }
  }

  static bool StartProcess(string path, string name)
  {
    string fullPath = GetFullPath(path);

    if (fullPath == "")
    {
      return false;
    }

    var psi = new ProcessStartInfo
    {
      FileName = "cmd.exe",
      Arguments = $"/c start \"{name}\" \"{fullPath}\"",
      UseShellExecute = true,
      CreateNoWindow = true,
      RedirectStandardOutput = false,
      RedirectStandardError = false
    };

    try
    {
      Process.Start(psi);
    }
    catch
    {
      return false;
    }

    Thread.Sleep(3200);

    if (Process.GetProcessesByName(name).Length > 0)
    {
      return true;
    } else
    {
      return false;
    }
  }
}