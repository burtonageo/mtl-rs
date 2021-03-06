use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use objc::runtime::BOOL;
use MTLDataType;

/// A `MTLVertexAttribute` object represents an attribute for per-vertex input
/// in a vertex shader function. You use vertex attribute objects to inspect
/// the inputs of a shader function by examining the `vertexAttributes` property
/// of the corresponding `MTLFunction` object.
pub trait MTLVertexAttribute {
    /// The name of the attribute. (read-only)
    unsafe fn name(self) -> id;

    /// The index of the attribute, as declared in Metal shader source code.
    /// (read-only)
    unsafe fn attributeIndex(self) -> NSUInteger;

    /// The data type for the attribute, as declared in Metal shader source code.
    /// (read-only)
    unsafe fn attributeType(self) -> MTLDataType;

    /// A Boolean value that indicates whether this vertex attribute is active.
    /// (read-only)
    ///
    /// #Discussion
    ///
    /// If NO, this attribute is inactive and can be ignored.
    unsafe fn active(self) -> BOOL;
}

impl MTLVertexAttribute for id {
    unsafe fn name(self) -> id {
        msg_send![self, name]
    }
    unsafe fn attributeIndex(self) -> NSUInteger {
        msg_send![self, attributeIndex]
    }
    unsafe fn attributeType(self) -> MTLDataType {
        msg_send![self, attributeType]
    }
    unsafe fn active(self) -> BOOL {
        msg_send![self, active]
    }
}
