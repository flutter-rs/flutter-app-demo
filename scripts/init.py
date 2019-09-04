#!/usr/bin/env python3
import os
import glob
import subprocess
from lib import look_for_proj_dir, tmpl_file
from lib.download import download_flutter_engine

def get_tmpl_config(proj_dir):
    tmplfile_path = os.path.join(proj_dir, '.tmplfiles')

    try:
        proj_name = input('What\'s the name of the project?\n')
    except KeyboardInterrupt:
        return

    lib_name = proj_name.replace('-', '_')
    try:
        with open(tmplfile_path) as f:
            tmplfiles = []
            for line in f.readlines():
                line = line.strip()

                if os.path.isdir(os.path.join(proj_dir, line)):
                    line = os.path.join(line, '*')

                tmplfiles.append(line)
    except:
        tmplfiles = []

    return (tmplfiles, {
        "name": proj_name,
        "lib_name": lib_name, # underlined version of name
    })


def install_py_deps(proj_dir):
    subprocess.run(
        ['pip3', 'install', '-r', './scripts/requirements.txt', '--user'],
        cwd = proj_dir, check = True)

def tmpl_proj(proj_dir):
    tmplfiles, config = get_tmpl_config(proj_dir)

    for pattern in tmplfiles:
        for fp in glob.iglob(os.path.join(proj_dir, pattern)):
            fp = os.path.join(proj_dir, fp)
            if os.path.isfile(fp):
                tmpl_file(fp, config)

def run():
    proj_dir = look_for_proj_dir(os.path.abspath(__file__), 'pubspec.yaml')
    tmplfile_path = os.path.join(proj_dir, '.tmplfiles')
    first_run = os.path.isfile(tmplfile_path)

    if first_run:
        print('🔮  Creating files')
        tmpl_proj(proj_dir)

        # remove tmplfile, useless now
        os.remove(tmplfile_path)

    print('🧩  Installing build dependencies')
    install_py_deps(proj_dir)

    print('🔮  Downloading flutter-engine')
    download_flutter_engine()

    print('🍭  Done! Happy coding.')

if __name__ == '__main__':
    run()
