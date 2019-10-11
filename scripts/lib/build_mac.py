import os
from shutil import copy, copyfile, copytree, rmtree
import subprocess

def prepare(envs):
    envs = dict(envs)
    APP_NAME = envs['NAME'] + '.app'
    APP_PATH = os.path.join(envs['OUTPUT_DIR'], APP_NAME)
    IDENTIFIER = envs['CONFIG']['mac']['identifier']

    envs.update(
        APP_NAME = APP_NAME,
        APP_PATH = APP_PATH,
        IDENTIFIER = IDENTIFIER,
    )
    return envs

def build(envs):
    app_path = envs.get('APP_PATH')
    output_dir = envs.get('OUTPUT_DIR')

    # clear last output
    rmtree(app_path, ignore_errors = True)

    bin_dir = os.path.join(app_path, 'Contents', 'MacOS')
    frm_dir = os.path.join(app_path, 'Contents', 'Frameworks')
    res_dir = os.path.join(app_path, 'Contents', 'Resources')

    os.makedirs(bin_dir, exist_ok = True)
    os.makedirs(res_dir, exist_ok = True)
    copy(os.path.join(output_dir, envs['NAME']), bin_dir)
    subprocess.run(['chmod', '+x', os.path.join(bin_dir, envs['NAME'])], check = True)
    copytree(os.path.join(output_dir, 'FlutterEmbedder.framework'), os.path.join(frm_dir, 'FlutterEmbedder.framework'), symlinks = True)
    # copy resources
    copy(os.path.join(envs['RUST_ASSETS_DIR'], 'icon.icns'), res_dir)
    copy(os.path.join(envs['RUST_ASSETS_DIR'], 'icudtl.dat'), res_dir)
    copytree(envs['FLUTTER_ASSETS'], os.path.join(res_dir, 'flutter_assets'))

    plist = plist_tmpl.format(
        identifier = envs['IDENTIFIER'],
        name = envs['NAME']
    )
    plist_file = open(os.path.join(app_path, 'Contents', 'Info.plist'), 'w+')
    plist_file.write(plist)

    return app_path


plist_tmpl = '''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>{name}</string>
	<key>CFBundleIconFile</key>
	<string>icon.icns</string>
	<key>CFBundleIdentifier</key>
	<string>{identifier}</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>LSUIElement</key>
	<true/>
</dict>
</plist>
'''
