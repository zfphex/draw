use std::{mem::transmute, panic::Location};

use winapi::shared::{
    ntdef::HRESULT,
    winerror::{S_FALSE, S_OK},
};

pub trait WinResult {
    fn unwrap(self);
}

impl WinResult for HRESULT {
    #[track_caller]
    fn unwrap(self) {
        if self == S_FALSE {
            let c = Location::caller();
            return println!(
                "{}:{}:{}: Successful but nonstandard completion (the precise meaning depends on context).", c.file(), c.line(), c.column(),
            );
        }

        if self == S_OK {
            return;
        }
        let result: u32 = unsafe { transmute(self) };
        match result {
                //D3D11_ERROR_FILE_NOT_FOUND
                0x887C0002 => panic!("D3D11_ERROR_FILE_NOT_FOUND - The file was not found."),
                0x887C0001 => panic!("D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS - There are too many unique instances of a particular type of state object."),
                0x887C0003 => panic!("D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS - There are too many unique instances of a particular type of view object."),
                0x887C0004 => panic!("D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD - The first call to ID3D11DeviceContext::Map after either ID3D11Device::CreateDeferredContext or ID3D11DeviceContext::FinishCommandList per Resource was not D3D11_MAP_WRITE_DISCARD."),
                0x887A0001 => panic!("DXGI_ERROR_INVALID_CALL - The method call is invalid. For example, a method's parameter may not be a valid pointer."),
                0x887A000A => panic!("DXGI_ERROR_WAS_STILL_DRAWING - The previous blit operation that is transferring information to or from this surface is incomplete."),
                0x80004005 => panic!("E_FAIL - Attempted to create a device with the debug layer enabled and the layer is not installed."),
                0x80070057 => panic!("E_INVALIDARG - An invalid parameter was passed to the returning function."),
                0x8007000E => panic!("E_OUTOFMEMORY - Direct3D could not allocate sufficient memory to complete the call."),
                0x80004001 => panic!("E_NOTIMPL - The method call isn't implemented with the passed parameter combination."),
                0x8007000A => panic!("E_PENDING - The data necessary to complete this operation is not yet available."),
                0x80004002 => panic!("E_POINTER - An invalid pointer was used."),
                0x80070006 => panic!("E_HANDLE - The handle is invalid."),
                0x80070005 => panic!("E_ACCESSDENIED - General access denied error."),
                0x800700AA => panic!("E_ABORT - Operation aborted."),
                0x800700DF => panic!("E_UNEXPECTED - Catastrophic failure."),
                0x8007007A => panic!("E_REMCLOSED - The object has been closed."),
                0x80070057 => panic!("E_INVALIDARG - One or more arguments are invalid."),
                0x80070070 => panic!("E_DISK_FULL - The disk is full."),
                0x8007000C => panic!("E_PAGENOTACTIVE - The page is not active."),
                0x80004003 => panic!("E_POINTER - Invalid pointer."),
                0x80070017 => panic!("E_NOTIMPL - Requested function is not implemented."),
                0x800700AA => panic!("E_ABORT - Operation aborted."),
                0x80070006 => panic!("E_HANDLE - Invalid handle."),
                0x8007001F => panic!("E_INVALIDARG - An invalid parameter was passed to the returning function."),
                0x8007000E => panic!("E_OUTOFMEMORY - Out of memory."),
                0x80070057 => panic!("E_INVALIDARG - Invalid argument."),
                0x80004005 => panic!("E_FAIL - Unspecified failure."),
                0x8007000D => panic!("E_PENDING - The data necessary to complete this operation is not yet available."),
                0x887A002B => panic!("DXGI_ERROR_ACCESS_DENIED - You tried to use a resource to which you did not have the required access privileges. This error is most typically caused when you write to a shared resource with read-only access."),
                0x887A0026 => panic!("DXGI_ERROR_ACCESS_LOST - The desktop duplication interface is invalid. The desktop duplication interface typically becomes invalid when a different type of image is displayed on the desktop."),
                0x887A0036 => panic!("DXGI_ERROR_ALREADY_EXISTS - The desired element already exists. This is returned by DXGIDeclareAdapterRemovalSupport if it is not the first time that the function is called."),
                0x887A002A => panic!("DXGI_ERROR_CANNOT_PROTECT_CONTENT - DXGI can't provide content protection on the swap chain. This error is typically caused by an older driver, or when you use a swap chain that is incompatible with content protection."),
                0x887A0006 => panic!("DXGI_ERROR_DEVICE_HUNG - The application's device failed due to badly formed commands sent by the application. This is a design-time issue that should be investigated and fixed."),
                0x887A0005 => panic!("DXGI_ERROR_DEVICE_REMOVED - The video card has been physically removed from the system, or a driver upgrade for the video card has occurred. The application should destroy and recreate the device."),
                0x887A0007 => panic!("DXGI_ERROR_DEVICE_RESET - The device failed due to a badly formed command. This is a run-time issue; The application should destroy and recreate the device."),
                0x887A0020 => panic!("DXGI_ERROR_DRIVER_INTERNAL_ERROR - The driver encountered a problem and was put into the device removed state."),
                0x887A000B => panic!("DXGI_ERROR_FRAME_STATISTICS_DISJOINT - An event (for example, a power cycle) interrupted the gathering of presentation statistics."),
                0x887A000C => panic!("DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE - The application attempted to acquire exclusive ownership of an output, but failed because some other application (or device within the application) already acquired ownership."),
                0x887A0001 => panic!("DXGI_ERROR_INVALID_CALL - The application provided invalid parameter data; this must be debugged and fixed before the application is released."),
                0x887A0003 => panic!("DXGI_ERROR_MORE_DATA - The buffer supplied by the application is not big enough to hold the requested data."),
                0x887A002C => panic!("DXGI_ERROR_NAME_ALREADY_EXISTS - The supplied name of a resource in a call to IDXGIResource1::CreateSharedHandle is already associated with some other resource."),
                0x887A0021 => panic!("DXGI_ERROR_NONEXCLUSIVE - A global counter resource is in use, and the Direct3D device can't currently use the counter resource."),
                0x887A0022 => panic!("DXGI_ERROR_NOT_CURRENTLY_AVAILABLE - The resource or request is not currently available, but it might become available later."),
                0x887A0002 => panic!("DXGI_ERROR_NOT_FOUND - When calling IDXGIObject::GetPrivateData, the GUID passed in is not recognized as one previously passed to IDXGIObject::SetPrivateData or IDXGIObject::SetPrivateDataInterface. When calling IDXGIFactory::EnumAdapters or IDXGIAdapter::EnumOutputs, the enumerated ordinal is out of range."),
                0x887A0023 => panic!("DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED - Reserved"),
                0x887A0024 => panic!("DXGI_ERROR_REMOTE_OUTOFMEMORY - Reserved"),
                0x887A0029 => panic!("DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE - The DXGI output (monitor) to which the swap chain content was restricted is now disconnected or changed."),
                0x887A002D => panic!("DXGI_ERROR_SDK_COMPONENT_MISSING - The operation depends on an SDK component that is missing or mismatched."),
                0x887A0028 => panic!("DXGI_ERROR_SESSION_DISCONNECTED - The Remote Desktop Services session is currently disconnected."),
                0x887A0004 => panic!("DXGI_ERROR_UNSUPPORTED - The requested functionality is not supported by the device or the driver."),
                0x887A0027 => panic!("DXGI_ERROR_WAIT_TIMEOUT - The time-out interval elapsed before the next desktop frame was available."),
                0x887A000A => panic!("DXGI_ERROR_WAS_STILL_DRAWING - The GPU was busy at the moment when a call was made to perform an operation, and did not execute or schedule the operation."),
                _ => panic!("{:#x}", self),
            }
    }
}
