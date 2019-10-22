#!/usr/bin/env python3
import os, sys
import re
import subprocess
import signal
import sys
import threading
import json
import argparse
from lib import look_for_proj_dir
from lib.utils import deep_update

def signal_handler(signal, frame):
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

FLUTTER = 'flutter.bat' if sys.platform == 'win32' else 'flutter'
PROJ_DIR = look_for_proj_dir(os.path.abspath(__file__), 'pubspec.yaml')
RUST_PROJ_DIR = os.path.join(PROJ_DIR, 'rust')

class CargoThread(threading.Thread):
    def __init__(self):
        super().__init__()
        self.observatory_uri = ''
        self.observatory_open = threading.Event()

    def run(self):
        self.proc = subprocess.Popen(['cargo', 'run'],
            stdout = subprocess.PIPE,
            cwd = RUST_PROJ_DIR
        )

        while True:
            line = self.proc.stdout.readline()
            if self.proc.poll() is not None:
                # proc ended
                return
            print(line.decode(), end = '')
            match = re.search(r'Observatory listening on (?P<schema>https?://)(\S*)', line.decode())

            if match:
                self.observatory_uri = match.group('schema') + match.group(2)
                self.observatory_open.set()


def cargo_run():
    cargo = CargoThread()
    cargo.start()
    cargo.observatory_open.wait()
    return cargo.observatory_uri

def write_vscode_launch(uri):
    vscode_dir = os.path.join(PROJ_DIR, '.vscode')
    # make sure .vscode dir exists
    os.makedirs(vscode_dir, exist_ok=True)

    vscode_json_file = os.path.join(vscode_dir, 'launch.json')

    conf = {
        'version': "0.2.0",
        "configurations": []
    }
    if os.path.exists(vscode_json_file):
        with open(vscode_json_file, 'r+') as f:
            deep_update(conf, json.load(f))
    print('existing conf:', conf)

    update_cmd(conf['configurations'], {
        "name": "Flutter-rs Attach",
        "request": "attach",
        "type": "dart",
        "observatoryUri": uri,
        "deviceId": "flutter-tester"
    })
    print('updated:', conf)


    with open(vscode_json_file, 'w+') as f:
        f.write(json.dumps(conf, indent=True))

def update_cmd(cmds, new):
    updated = False
    for cmd in cmds:
        if cmd['name'] == new['name']:
            updated = True
            deep_update(cmd, new)
    if not updated:
        cmds.append(new)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(prog='run', description='Debug flutter-rs app')
    parser.add_argument('--vscode', action='store_true', help='Generate vscode launch.json file')

    args = parser.parse_args()

    print('🍀  Building flutter bundle')
    subprocess.run(
        [FLUTTER, 'build', 'bundle', '--track-widget-creation'],
        cwd = PROJ_DIR, check = True)

    print('🦀  Building rust project')
    uri = cargo_run()
    if not uri:
        raise Exception('Launch cargo error')

    if args.vscode:
        write_vscode_launch(uri)
        print('🍹  Now run "flutter attach" from vscode debug panel to debug')
    else:
        print('🍹  Attaching dart debugger')
        subprocess.run(
            [FLUTTER, 'attach', '--device-id=flutter-tester', '--debug-uri=' + uri],
            cwd = PROJ_DIR, check = True)
