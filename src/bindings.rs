#![allow(non_camel_case_types, non_snake_case)]

#[repr(C)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: NTSTATUS,
    pub Pointer: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}

pub type HANDLE = isize;
pub type NTSTATUS = i32;
pub type BOOL = i32;

pub const FSCTL_GET_REPARSE_POINT: u32 = 589992;
pub const IO_REPARSE_TAG_MOUNT_POINT: u32 = 2684354563;
pub const IO_REPARSE_TAG_SYMLINK: u32 = 2684354572;

pub type GETFINALPATHNAMEBYHANDLE_FLAGS = u32;

pub type HMODULE = isize;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;

#[repr(C)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
#[repr(C)]
pub union OVERLAPPED_0 {
    pub Anonymous: std::mem::ManuallyDrop<OVERLAPPED_0_0>,
    pub Pointer: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: HANDLE,
}

pub type FILE_SHARE_MODE = u32;
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = 4;
pub const FILE_SHARE_READ: FILE_SHARE_MODE = 1;
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2;

pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = 2;
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES = 2147483648;
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = 1073741824;
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES = 536870912;
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = 268435456;
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES = 134217728;
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES = 67108864;
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = 33554432;
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = 16777216;
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES = 8388608;
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = 2097152;
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES = 1048576;

pub type FILE_ACCESS_RIGHTS = u32;
pub const FILE_READ_DATA: FILE_ACCESS_RIGHTS = 1;
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_RIGHTS = 1;
pub const FILE_WRITE_DATA: FILE_ACCESS_RIGHTS = 2;
pub const FILE_ADD_FILE: FILE_ACCESS_RIGHTS = 2;
pub const FILE_APPEND_DATA: FILE_ACCESS_RIGHTS = 4;
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_RIGHTS = 4;
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_RIGHTS = 4;
pub const FILE_READ_EA: FILE_ACCESS_RIGHTS = 8;
pub const FILE_WRITE_EA: FILE_ACCESS_RIGHTS = 16;
pub const FILE_EXECUTE: FILE_ACCESS_RIGHTS = 32;
pub const FILE_TRAVERSE: FILE_ACCESS_RIGHTS = 32;
pub const FILE_DELETE_CHILD: FILE_ACCESS_RIGHTS = 64;
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_RIGHTS = 128;
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_RIGHTS = 256;
pub const DELETE: FILE_ACCESS_RIGHTS = 65536;
pub const READ_CONTROL: FILE_ACCESS_RIGHTS = 131072;
pub const WRITE_DAC: FILE_ACCESS_RIGHTS = 262144;
pub const WRITE_OWNER: FILE_ACCESS_RIGHTS = 524288;
pub const SYNCHRONIZE: FILE_ACCESS_RIGHTS = 1048576;
pub const FILE_ALL_ACCESS: FILE_ACCESS_RIGHTS = 2032127;
pub const FILE_GENERIC_READ: FILE_ACCESS_RIGHTS = 1179785;
pub const FILE_GENERIC_WRITE: FILE_ACCESS_RIGHTS = 1179926;
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_RIGHTS = 1179808;

pub type GENERIC_ACCESS_RIGHTS = u32;
pub const GENERIC_READ: GENERIC_ACCESS_RIGHTS = 2147483648;
pub const GENERIC_WRITE: GENERIC_ACCESS_RIGHTS = 1073741824;
pub const GENERIC_EXECUTE: GENERIC_ACCESS_RIGHTS = 536870912;
pub const GENERIC_ALL: GENERIC_ACCESS_RIGHTS = 268435456;

pub const STATUS_SUCCESS: i32 = 0;

pub type WIN32_ERROR = u32;
pub const NO_ERROR: WIN32_ERROR = 0;
pub const ERROR_BUFFER_OVERFLOW: WIN32_ERROR = 111;

pub const INVALID_HANDLE_VALUE: isize = -1;

pub const ACCESS_SYSTEM_SECURITY: u32 = 16777216;
pub const MAXIMUM_ALLOWED: u32 = 33554432;

// #[minwin]
// mod bindings {}

pub type FILE_TYPE = u32;
pub const FILE_TYPE_UNKNOWN: FILE_TYPE = 0;
pub const FILE_TYPE_DISK: FILE_TYPE = 1;
pub const FILE_TYPE_CHAR: FILE_TYPE = 2;
pub const FILE_TYPE_PIPE: FILE_TYPE = 3;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}

#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: *const u8) -> HMODULE;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: *const u8) -> FARPROC;
    pub fn GetFinalPathNameByHandleW(
        hFile: HANDLE,
        lpszFilePath: *mut u16,
        cchFilePath: u32,
        dwFlags: GETFINALPATHNAMEBYHANDLE_FLAGS,
    ) -> u32;
    pub fn GetFullPathNameW(
        lpFileName: *const u16,
        nBufferLength: u32,
        lpBuffer: *mut u16,
        lpFilePart: *mut *mut u16,
    ) -> u32;
    pub fn DeviceIoControl(
        hDevice: HANDLE,
        dwIoControlCode: u32,
        lpInBuffer: *const std::ffi::c_void,
        nInBufferSize: u32,
        lpOutBuffer: *mut std::ffi::c_void,
        nOutBufferSize: u32,
        lpBytesReturned: *mut u32,
        lpOverlapped: *mut OVERLAPPED,
    ) -> BOOL;
    pub fn ReOpenFile(
        hOriginalFile: HANDLE,
        dwDesiredAccess: u32,
        dwShareMode: FILE_SHARE_MODE,
        dwFlagsAndAttributes: FILE_FLAGS_AND_ATTRIBUTES,
    ) -> HANDLE;
    pub fn GetFileInformationByHandle(
        hFile: HANDLE,
        lpFileInformation: *mut BY_HANDLE_FILE_INFORMATION,
    ) -> BOOL;
    pub fn GetFileType(hFile: HANDLE) -> FILE_TYPE;
    pub fn GetLastError() -> WIN32_ERROR;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> BOOL;
}

#[link(name = "ntdll", kind = "raw-dylib")]
extern "system" {
    pub fn RtlNtStatusToDosError(Status: NTSTATUS) -> u32;
}
