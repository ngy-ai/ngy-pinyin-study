plugins {
    id("com.android.application")
}

android {
    namespace = "com.ngy.pinyin"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.ngy.pinyin"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        ndk {
            // arm64 给真机，x86_64 给模拟器
            abiFilters += listOf("arm64-v8a", "x86_64")
        }
    }

    // cargo-ndk 把 libpinyin_study_mobile.so 输出到这里
    sourceSets["main"].jniLibs.srcDir("src/main/jniLibs")

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
}
