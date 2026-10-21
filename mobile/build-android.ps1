# Build the Android APK for pinyin-study-mobile.
#
#   .\build-android.ps1                    # arm64 debug APK
#   .\build-android.ps1 -Abi x86_64        # emulator
#   .\build-android.ps1 -Abi all           # arm64 + x86_64
#   .\build-android.ps1 -Release           # release APK
#   .\build-android.ps1 -SkipRust          # reuse the existing .so, only run gradle
#
# Requirements: rustup, Android SDK + NDK, JDK 17+, gradle.
#
# The first Rust build pulls gpui from the Zed repository (hundreds of MB) and
# compiles wgpu, so expect it to take a while.

param(
    [ValidateSet("arm64-v8a", "x86_64", "all")]
    [string]$Abi = "arm64-v8a",
    [switch]$Release,
    [switch]$SkipRust
)

$ErrorActionPreference = "Stop"
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location $here
try {
    # --- NDK -----------------------------------------------------------------
    if (-not $env:ANDROID_NDK_HOME -or -not (Test-Path $env:ANDROID_NDK_HOME)) {
        $sdk = if ($env:ANDROID_HOME) { $env:ANDROID_HOME } else { $env:ANDROID_SDK_ROOT }
        if (-not $sdk) { throw "ANDROID_HOME is not set" }
        $ndk = Get-ChildItem (Join-Path $sdk "ndk") -Directory -ErrorAction SilentlyContinue |
            Sort-Object Name -Descending | Select-Object -First 1
        if (-not $ndk) { throw "no NDK found under $sdk\ndk" }
        $env:ANDROID_NDK_HOME = $ndk.FullName
    }
    Write-Host "NDK: $env:ANDROID_NDK_HOME"

    $bin = Join-Path $env:ANDROID_NDK_HOME "toolchains\llvm\prebuilt\windows-x86_64\bin"
    $clang = Join-Path $bin "aarch64-linux-android26-clang.cmd"
    if (-not (Test-Path $clang)) { throw "NDK toolchain not found at $bin" }

    $api = 26
    $abis = if ($Abi -eq "all") { @("arm64-v8a", "x86_64") } else { @($Abi) }

    $targets = @{
        "arm64-v8a" = @{ Rust = "aarch64-linux-android"; Cc = "aarch64-linux-android$api"; Env = "AARCH64_LINUX_ANDROID" }
        "x86_64"    = @{ Rust = "x86_64-linux-android"; Cc = "x86_64-linux-android$api"; Env = "X86_64_LINUX_ANDROID" }
    }

    # 交叉编译需要这几个变量：cc-rs 找 CC_/CXX_/AR_，rustc 找 CARGO_TARGET_<T>_LINKER
    $env:RUST_FONTCONFIG_DLOPEN = "on"

    $profileArgs = @()
    if ($Release) { $profileArgs += "--release" }
    $profileDir = if ($Release) { "release" } else { "debug" }

    foreach ($abi in $abis) {
        $t = $targets[$abi]
        if (-not $t) { throw "unsupported ABI: $abi" }

        Set-Item -Path "Env:CC_$($t.Env)" -Value (Join-Path $bin "$($t.Cc)-clang.cmd")
        Set-Item -Path "Env:CXX_$($t.Env)" -Value (Join-Path $bin "$($t.Cc)-clang++.cmd")
        Set-Item -Path "Env:AR_$($t.Env)" -Value (Join-Path $bin "llvm-ar.exe")

        if (-not $SkipRust) {
            rustup target add $t.Rust
            Write-Host "==> cargo build --target $($t.Rust)"
            # rustc 的 linker 变量名：CARGO_TARGET_<TARGET>_LINKER
            $linkerVar = "CARGO_TARGET_" + $t.Rust.ToUpper().Replace("-", "_") + "_LINKER"
            Set-Item -Path "Env:$linkerVar" -Value (Join-Path $bin "$($t.Cc)-clang.cmd")

            cargo build --target $t.Rust @profileArgs
            if ($LASTEXITCODE -ne 0) { throw "cargo build failed for $abi" }
        }

        $out = Join-Path $here "android\app\src\main\jniLibs\$abi"
        New-Item -ItemType Directory -Force $out | Out-Null
        $so = "target\$($t.Rust)\$profileDir\libpinyin_study_mobile.so"
        if (-not (Test-Path $so)) { throw "missing $so" }
        Copy-Item $so $out -Force

        # debug 包有 460MB，strip 之后 ~35MB
        & (Join-Path $bin "llvm-strip.exe") --strip-unneeded (Join-Path $out "libpinyin_study_mobile.so")
    }

    # --- gradle --------------------------------------------------------------
    if (-not (Test-Path "android/gradlew.bat")) {
        Push-Location android
        try {
            gradle wrapper --gradle-version 8.9
            if ($LASTEXITCODE -ne 0) { throw "gradle wrapper failed" }
        } finally {
            Pop-Location
        }
    }

    $task = if ($Release) { "assembleRelease" } else { "assembleDebug" }
    & "android/gradlew.bat" -p android $task
    if ($LASTEXITCODE -ne 0) { throw "gradle $task failed" }

    $apk = Get-ChildItem "android/app/build/outputs/apk" -Recurse -Filter "*.apk" |
        Sort-Object LastWriteTime -Descending | Select-Object -First 1
    Write-Host ""
    Write-Host "APK: $($apk.FullName)"
    Write-Host "Install: adb install -r `"$($apk.FullName)`""
} finally {
    Pop-Location
}
