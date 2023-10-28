`{
    "version": 3,
    "cmakeMinimumRequired": {
        "major": 3,
        "minor": 21,
        "patch": 0
    },
    "configurePresets": [
        {
            "name": "gcc",
            "displayName": "GCC",
            "generator": "Ninja Multi-Config",
            "toolchainFile": "./cmake/toolchain-gcc-${info.target_cpu}.cmake",
            "binaryDir": "./build"
        }
    ],
    "buildPresets": [
        {
            "name": "release",
            "displayName": "Release",
            "configurePreset": "gcc",
            "configuration": "Release"
        },
        {
            "name": "debug",
            "displayName": "Debug",
            "configurePreset": "gcc",
            "configuration": "Debug"
        }
    ],
    "testPresets": []
}`