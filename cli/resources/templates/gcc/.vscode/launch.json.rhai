`{
    "configurations": [
        {
            "cwd": "` + "${workspaceRoot}" + `",
            "executable": "` + "${command:cmake.launchTargetPath}" + `",
            "name": "Debug (OpenOCD)",
            "request": "launch",
            "type": "cortex-debug",
            "servertype": "openocd",
            "configFiles": [
                "openocd.cfg"
            ],
            "searchDir": [],
            "showDevDebugOutput": "none",
            "deviceName": "${info.target_mcu}"
        }
    ]
}`