use std::env;
use std::path::PathBuf;

const CORECLR_ROOT_VAR_NAME: &'static str = "CORECLR_SYS_CORECLR_ROOT";
const SDK_ROOT_VAR_NAME: &'static str = "CORECLR_SYS_SDK_ROOT";
const SHARED_FRAMEWORK_NAME: &'static str = "Microsoft.NETCore.App";
const VERSION: &'static str = "1.0.0";

#[cfg(target_family = "unix")]
mod os {
    use std::env;
    use std::path::PathBuf;

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub const CORECLR_LIB_NAME: &'static str = "libcoreclr.dylib";

    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    pub const CORECLR_LIB_NAME: &'static str = "libcoreclr.so";

    pub fn get_global_sdk_path() -> PathBuf {
        PathBuf::from("/usr/share/dotnet")
    }

    pub fn get_user_sdk_path() -> Option<PathBuf> {
        match env::var("HOME") {
            Ok(ref v) if !v.is_empty() => {
                let mut home = PathBuf::from(v);
                home.push(".dotnet");
                Some(home)
            },
            _ => None
        }
    }
}

fn main() {
    // First, allow override of the entire path
    let coreclr_root = match env::var(CORECLR_ROOT_VAR_NAME) {
        Ok(ref v) if !v.is_empty() => PathBuf::from(v),
        _ => find_coreclr_root()
    };

    // Check that the library is there
    let libcoreclr = coreclr_root.join(os::CORECLR_LIB_NAME);
    if !libcoreclr.exists() || !libcoreclr.is_file() {
        panic!("CoreCLR root '{}' exists, but '{}' could not be found in that directory", coreclr_root.to_str().unwrap(), os::CORECLR_LIB_NAME);
    }

    println!("cargo:rustc-link-search=native={}", coreclr_root.to_str().unwrap());
    println!("cargo:rustc-link-lib=coreclr");
}

fn find_coreclr_root() -> PathBuf {
    // Try to find the SDK
    let mut sdk_root = match env::var(SDK_ROOT_VAR_NAME) {
        Ok(ref v) if !v.is_empty() => PathBuf::from(v),
        _ => find_sdk_root()
    };
    sdk_root.push("shared");
    sdk_root.push(SHARED_FRAMEWORK_NAME);
    sdk_root.push(VERSION);

    if sdk_root.exists() && sdk_root.is_dir() {
        sdk_root
    } else {
        panic!("SDK exists, but expected shared framework '{} {}' could not be found at '{}'", SHARED_FRAMEWORK_NAME, VERSION, sdk_root.to_str().unwrap());
    }
}

fn find_sdk_root() -> PathBuf {
    // Try the global root
    let global = os::get_global_sdk_path();
    let user = os::get_user_sdk_path();
    if global.exists() && global.is_dir() {
        global
    } else if user.is_some() {
        let user = user.unwrap();
        if user.exists() && user.is_dir() {
            user
        }
        else {
            panic!("Could not find SDK root, tried global path: '{}' and user path '{}'. Try setting the environment variable '{}' to the root of the SDK, or '{}' to the directory containing {}.", global.to_str().unwrap(), user.to_str().unwrap(), SDK_ROOT_VAR_NAME, CORECLR_ROOT_VAR_NAME, os::CORECLR_LIB_NAME)
        }
    } else {
        panic!("Could not find SDK root, tried global path: '{}'. Try setting the environment variable '{}' to the root of the SDK, or '{}' to the directory containing {}.", global.to_str().unwrap(), SDK_ROOT_VAR_NAME, CORECLR_ROOT_VAR_NAME, os::CORECLR_LIB_NAME)
    }
}
