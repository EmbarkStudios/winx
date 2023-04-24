#![allow(non_camel_case_types)]

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

#[repr(C)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: u32,
}

#[repr(C)]
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}

pub type FILE_INFORMATION_CLASS = i32;
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = 1;

pub const FSCTL_GET_REPARSE_POINT: u32 = 589992;
pub const IO_REPARSE_TAG_MOUNT_POINT: i32 = -1610612733;
pub const IO_REPARSE_TAG_SYMLINK: i32 = -1610612724;

pub type GETFINALPATHNAMEBYHANDLE_FLAGS = u32;
pub const VOLUME_NAME_DOS: GETFINALPATHNAMEBYHANDLE_FLAGS = 0;
pub const VOLUME_NAME_GUID: GETFINALPATHNAMEBYHANDLE_FLAGS = 1;
pub const VOLUME_NAME_NT: GETFINALPATHNAMEBYHANDLE_FLAGS = 2;
pub const VOLUME_NAME_NONE: GETFINALPATHNAMEBYHANDLE_FLAGS = 4;
pub const FILE_NAME_NORMALIZED: GETFINALPATHNAMEBYHANDLE_FLAGS = 0;
pub const FILE_NAME_OPENED: GETFINALPATHNAMEBYHANDLE_FLAGS = 8;

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
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = 0;
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = 4;
pub const FILE_SHARE_READ: FILE_SHARE_MODE = 1;
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2;

pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = 1;
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = 2;
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = 4;
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = 16;
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = 32;
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = 64;
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = 128;
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = 256;
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = 512;
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = 1024;
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = 2048;
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = 4096;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = 8192;
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = 16384;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = 32768;
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = 65536;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = 131072;
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = 262144;
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = 524288;
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES = 1048576;
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = 262144;
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = 4194304;
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
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = 524288;
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = 3;
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = 1;
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = 2;
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = 0;
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = 65536;
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = 131072;
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = 196608;
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES = 262144;
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = 524288;
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = 1048576;
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = 2031616;

pub type WIN32_ERROR = u32;
pub const NO_ERROR: WIN32_ERROR = 0;

pub type FILE_TYPE = u32;
pub const FILE_TYPE_UNKNOWN: FILE_TYPE = 0;
pub const FILE_TYPE_DISK: FILE_TYPE = 1;
pub const FILE_TYPE_CHAR: FILE_TYPE = 2;
pub const FILE_TYPE_PIPE: FILE_TYPE = 3;
pub const FILE_TYPE_REMOTE: FILE_TYPE = 32768;

#[derive(Clone)]
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

pub const FILE_ATTRIBUTE_HIDDEN: u32 = 2;

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
