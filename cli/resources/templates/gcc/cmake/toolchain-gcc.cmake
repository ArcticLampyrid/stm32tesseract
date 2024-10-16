function(setup_arm_embeded_gcc_toolchain mcpu float_abi fpu)
    set(CMAKE_SYSTEM_NAME Generic PARENT_SCOPE)
    set(CMAKE_SYSTEM_PROCESSOR arm PARENT_SCOPE)
    set(CMAKE_SYSTEM_VERSION 1 PARENT_SCOPE)

    set(CMAKE_C_COMPILER arm-none-eabi-gcc PARENT_SCOPE)
    set(CMAKE_CXX_COMPILER arm-none-eabi-g++ PARENT_SCOPE)
    set(CMAKE_ASM_COMPILER arm-none-eabi-gcc PARENT_SCOPE)
    set(CMAKE_AR arm-none-eabi-gcc-ar PARENT_SCOPE)
    set(CMAKE_NM arm-none-eabi-gcc-nm PARENT_SCOPE)
    set(CMAKE_RANLIB arm-none-eabi-gcc-ranlib PARENT_SCOPE)
    set(CMAKE_OBJCOPY arm-none-eabi-objcopy PARENT_SCOPE)
    set(CMAKE_OBJDUMP arm-none-eabi-objdump PARENT_SCOPE)
    set(SIZE arm-none-eabi-size PARENT_SCOPE)
    set(CMAKE_EXECUTABLE_SUFFIX ".elf" PARENT_SCOPE)
    set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY PARENT_SCOPE)

    set(__common_flags "-mfloat-abi=${float_abi} -mfpu=${fpu} -mcpu=${mcpu} -mthumb -mthumb-interwork -ffunction-sections -fdata-sections -fno-common -fmessage-length=0")
    set(CMAKE_C_FLAGS "${__common_flags}" PARENT_SCOPE)
    set(CMAKE_CXX_FLAGS "${__common_flags} -fno-rtti -fno-exceptions -fno-threadsafe-statics" PARENT_SCOPE)
    set(CMAKE_ASM_FLAGS "${__common_flags} -x assembler-with-cpp" PARENT_SCOPE)
    set(CMAKE_EXE_LINKER_FLAGS "-mcpu=${mcpu} -mthumb -mthumb-interwork -mfpu=${fpu} -mfloat-abi=${float_abi} -specs=nano.specs -Wl,--gc-sections -u _printf_float" PARENT_SCOPE)

    set(CMAKE_C_FLAGS_DEBUG "-Og -g" PARENT_SCOPE)
    set(CMAKE_CXX_FLAGS_DEBUG "-Og -g" PARENT_SCOPE)
    set(CMAKE_ASM_FLAGS_DEBUG "-g" PARENT_SCOPE)
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "" PARENT_SCOPE)

    set(CMAKE_C_FLAGS_RELWITHDEBINFO "-Ofast -g -flto" PARENT_SCOPE)
    set(CMAKE_CXX_FLAGS_RELWITHDEBINFO "-Ofast -g -flto" PARENT_SCOPE)
    set(CMAKE_ASM_FLAGS_RELWITHDEBINFO "-g" PARENT_SCOPE)
    set(CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO "-flto" PARENT_SCOPE)

    set(CMAKE_C_FLAGS_MINSIZEREL "-Os -flto" PARENT_SCOPE)
    set(CMAKE_CXX_FLAGS_MINSIZEREL "-Os -flto" PARENT_SCOPE)
    set(CMAKE_ASM_FLAGS_MINSIZEREL "" PARENT_SCOPE)
    set(CMAKE_EXE_LINKER_FLAGS_MINSIZEREL "-flto" PARENT_SCOPE)

    set(CMAKE_C_FLAGS_RELEASE "-Ofast -flto" PARENT_SCOPE)
    set(CMAKE_CXX_FLAGS_RELEASE "-Ofast -flto" PARENT_SCOPE)
    set(CMAKE_ASM_FLAGS_RELEASE "" PARENT_SCOPE)
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "-flto" PARENT_SCOPE)
endfunction()