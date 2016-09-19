// coreclr-sys - Low-level FFI bindings for coreclr.dll/libcoreclr.so/libcoreclr.dylib

pub type HostHandle = *const u8;
pub type Delegate = *const u8;
pub type AppDomainId = usize;
pub type HResult = usize;

/// Represents the type of the `coreclr_initialize` function in the CoreCLR library
///
/// # Parameters
/// * `exe_path` - UTF-8-encoded path to the managed executable that will be run in this session.
/// * `app_domain_friendly_name` - UTF-8-encoded friendly name for the CLR Application Domain.
/// * `property_keys` - C-style Array of UTF-8-encoded property names, exactly `property_count`
/// items are expected
/// * `property_values` - C-style Array of UTF-8-encoded property values, exactly `property_count
/// items are expected (matched up 1-to-1 with the names in `property_keys`
/// * `host_handle` - C-style output parameter which receives a handle to the CoreCLR Host object
/// * `domain_id` - C-style output parameter which receives a `usize` representing the ID of the
/// CLR Application Domain created.
pub type InitializeFn = extern "C" fn initialize(
    exe_path: *const u8,
    app_domain_friendly_name: *const u8,
    property_keys: *const *const u8,
    property_values: *const *const u8,
    property_count: isize,
    host_handle: *mut HostHandle
    domain_id: *mut AppDomainId) -> HResult;

/// Represents the type of the `coreclr_shutdown` function in the CoreCLR library
///
/// # Parameters
/// * `host_handle` - the Host handle provided by `initialize`
/// * `domain_id` - The Application Domain ID provided by `initialize`.
pub type ShutdownFn = extern "C" fn shutdown(host_handle: HostHandle, domain_id: AppDomainId) -> HResult;

/// Represents the type of the `coreclr_execute_assembly` function in the CoreCLR library
///
/// # Parameters
/// * `host_handle` - the Host handle provided by `initialize`
/// * `domain_id` - The Application Domain ID provided by `initialize`.
/// * `argc` - The number of arguments provided in the `argv` array
/// * `argv` - C-style array of UTF-8-encoded strings representing arguments to the application
/// * `managed_assembly_path` - UTF-8-encoded path to the managed assembly to run
/// * `exit_code` - C-style output parameter which receives the exit code of the application.
pub type ExecuteAssemblyFn = extern "C" fn execute_assembly(
    host_handle: HostHandle,
    domain_id: AppDomainId,
    argc: isize,
    argv: *const *const u8,
    managed_assembly_path: *const u8,
    exit_code: *mut usize) -> HResult;

/// Represents the type of the `coreclr_create_delegate` function in the CoreCLR library
///
/// # Parameters
/// * `host_handle` - the Host handle provided by `initialize`
/// * `domain_id` - The Application Domain ID provided by `initialize`.
/// * `assembly_name` - UTF-8-encoded name of the assembly containing the method to create a
/// delegate for
/// * `type_name` - UTF-8-encoded name of the type containing the method to create a delegate
/// for
/// * `method_name` - UTF-8-encoded name of the method to create a delegate
/// for
/// * `delegate` - C-style output parameter which receives the pointer to the function to call in
/// order to invoke the delegate.
pub type CreateDelegateFn = extern "C" fn create_delegate(
    host_handle: HostHandle,
    domain_id: AppDomainId,
    assembly_name: *const u8,
    type_name: *const u8,
    method_name: *const u8,
    delegate: *mut Delegate);
