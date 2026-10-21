plugins {
    id("com.android.application")
}

android {
    namespace = "com.ngy.pinyin"
    compileSdk = 35

    // 打哪个架构由 -Pabis=arm64-v8a 决定，默认两个都带上
    val abis = (project.findProperty("abis") as String?)
        ?.split(",")
        ?.map { it.trim() }
        ?.filter { it.isNotEmpty() }
        ?: listOf("arm64-v8a", "x86_64")

    defaultConfig {
        applicationId = "com.ngy.pinyin"
        minSdk = 26
        // 35 会强制 edge-to-edge，界面会画到状态栏下面（时钟压住标题）。
        // 34 保持系统自动留出安全区，等处理完 insets 再抬上去。
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0"

        ndk {
            abiFilters += abis
        }
    }

    // build-android.ps1 把 libpinyin_study_mobile.so 输出到这里
    sourceSets["main"].jniLibs.srcDir("src/main/jniLibs")

    signingConfigs {
        // 只是装到手机上试，用本机的 debug 签名；上架要换成正式 keystore
        create("dev") {
            storeFile = File(System.getProperty("user.home"), ".android/debug.keystore")
            storePassword = "android"
            keyAlias = "androiddebugkey"
            keyPassword = "android"
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            signingConfig = signingConfigs.getByName("dev")
        }
    }
}
