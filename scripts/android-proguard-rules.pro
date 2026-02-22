# ProGuard/R8 rules to satisfy R8 warnings about implicit default constructors
# Maintained in repo and copied into generated app by post-dx-patch.sh

# Keep default constructors explicitly for classes mentioned in warnings
-keep public class com.google.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.android.vending.licensing.ILicensingService { void <init>(); }
-keep public class com.google.android.vending.licensing.ILicensingService { void <init>(); }
-keep class android.support.annotation.Keep { void <init>(); }
-keep class androidx.annotation.Keep { void <init>(); }
-keep class androidx.webkit.WebViewClientCompat { void <init>(); }
-keep class androidx.versionedparcelable.ParcelImpl { void <init>(); }
-keep class * extends androidx.startup.Initializer { void <init>(); }

# Additional conservative keep rules for versionedparcelable and lifecycle
-keep class * implements androidx.versionedparcelable.VersionedParcelable { *; }
-keep class * implements androidx.lifecycle.LifecycleObserver { *; }
