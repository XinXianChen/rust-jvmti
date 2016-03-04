use super::super::capabilities::Capabilities;
use super::super::error::NativeError;
use super::super::version::VersionNumber;
use super::super::native::JVMTIEnvPtr;

pub trait JVMTI {

    ///
    /// Return the JVM TI version number, which includes major, minor and micro version numbers.
    ///
    fn get_version_number(&self) -> VersionNumber;
    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    fn add_capabilities(&self, new_capabilities: Capabilities) -> Result<Capabilities, NativeError>;
    fn get_capabilities(&self) -> Capabilities;
}

pub struct JVMTIEnvironment {

    jvmti: JVMTIEnvPtr
}

impl JVMTIEnvironment {
    pub fn new(env_ptr: JVMTIEnvPtr) -> JVMTIEnvironment {
        JVMTIEnvironment { jvmti: env_ptr }
    }
}

impl JVMTI for JVMTIEnvironment {

    fn get_version_number(&self) -> VersionNumber {
        unsafe {
            let mut version: i32 = 0;
            let version_ptr = &mut version;
            (**self.jvmti).GetVersionNumber.unwrap()(self.jvmti, version_ptr);
            let uversion = *version_ptr as u32;
            VersionNumber::from_u32(&uversion)
        }
    }

    fn add_capabilities(&self, new_capabilities: Capabilities) -> Result<Capabilities, NativeError> {
        Err(NativeError::NoError)
    }

    fn get_capabilities(&self) -> Capabilities {
        unsafe {
            let caps = Capabilities::new();
            let mut native_caps = caps.to_native();
            {
                let cap_ptr = &mut native_caps;
                (**self.jvmti).GetCapabilities.unwrap()(self.jvmti, cap_ptr);
            }
            Capabilities::from_native(&native_caps)
        }
    }
}