import os
import sys
import subprocess
import platform

def run(system: str, path: str):
  if system == 'linux':
    os.system(f'gnome-terminal -- {path}')
  elif system == 'windows':
    subprocess.run(['cmd.exe', '/c', 'start', '', path])

if __name__ == '__main__':
  run(platform.system().lower(), sys.argv[1])