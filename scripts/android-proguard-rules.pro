# ProGuard/R8 rules — maintained in repo, copied into generated app by post-dx-patch.sh

# ── Licensing stubs (suppress R8 missing-class warnings) ───────────────────
-keep public class com.google.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.android.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.google.android.vending.licensing.ILicensingService { void <init>(); }
-keep class android.support.annotation.Keep { void <init>(); }
-keep class androidx.annotation.Keep { void <init>(); }

# ── AndroidX / WebKit ───────────────────────────────────────────────────────
-keep class androidx.webkit.** { *; }
-keep class androidx.webkit.WebViewClientCompat { void <init>(); }
-keep class androidx.versionedparcelable.ParcelImpl { void <init>(); }
-keep class * extends androidx.startup.Initializer { void <init>(); }
-keep class * implements androidx.versionedparcelable.VersionedParcelable { *; }
-keep class * implements androidx.lifecycle.LifecycleObserver { *; }

# Explicit keeps to silence R8 warnings in default proguard files
# (add member pattern { void <init>(); } as recommended by R8)
-keep public class com.google.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.android.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.google.android.vending.licensing.ILicensingService { void <init>(); }
-keep class android.support.annotation.Keep { void <init>(); }
-keep,allowshrinking public class androidx.webkit.WebViewClientCompat { void <init>(); }
-keep public class androidx.versionedparcelable.ParcelImpl { void <init>(); }

# ── Wry / Dioxus native bridge ──────────────────────────────────────────────
# WryActivity and all its subclasses (including our MainActivity) must survive
# R8 because they are instantiated by the Android framework via reflection.
-keep class dev.dioxus.** { *; }
-keep class com.tfsports.** { *; }
-keep public class * extends android.app.Activity { *; }
-keep public class * extends androidx.appcompat.app.AppCompatActivity { *; }

# The native library bridge is loaded via meta-data in AndroidManifest;
# keep JNI-exported symbols and do not rename them.
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep OnBackPressedCallback used in our MainActivity patch
-keep class androidx.activity.OnBackPressedCallback { *; }
-keep class androidx.activity.OnBackPressedDispatcher { *; }

# Suppress warnings for classes only present on some SDK versions
-dontwarn android.webkit.**
-dontwarn androidx.webkit.**
