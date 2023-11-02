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
            "binaryDir": "./build",
            "cacheVariables": {
                "CMAKE_CONFIGURATION_TYPES": "Debug;Release;MinSizeRel;RelWithDebInfo"
            },
            "warnings": {
                "unusedCli": false
            }
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
        },
        {
            "name": "min_size_rel",
            "displayName": "MinSizeRel",
            "configurePreset": "gcc",
            "configuration": "MinSizeRel"
        },
        {
            "name": "rel_with_deb_info",
            "displayName": "RelWithDebInfo",
            "configurePreset": "gcc",
            "configuration": "RelWithDebInfo"
        }
    ],
    "testPresets": []
}`