let jlink_device = info.target_mcu;
jlink_device.pop(2);

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
        },
        {
            "cwd": "` + "${workspaceRoot}" + `",
            "executable": "` + "${command:cmake.launchTargetPath}" + `",
            "name": "Debug (JLink)",
            "request": "launch",
            "type": "cortex-debug",
            "servertype": "jlink",
            "serverpath": "C:/Program Files/SEGGER/JLink/JLinkGDBServerCL.exe",
            "interface": "swd",
            "device": "${jlink_device}",
            "showDevDebugOutput": "none",
            "deviceName": "${info.target_mcu}"
        }
    ]
}`