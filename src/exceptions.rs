use pyo3::{
    create_exception,
    exceptions::{PyException, PyIOError, PyValueError},
    prelude::*,
};

pub(super) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("QiniuCallbackError", py.get_type::<QiniuCallbackError>())?;
    m.add("QiniuUnknownError", py.get_type::<QiniuUnknownError>())?;
    m.add(
        "QiniuInvalidURLError",
        py.get_type::<QiniuInvalidURLError>(),
    )?;
    m.add(
        "QiniuInvalidHeaderName",
        py.get_type::<QiniuInvalidHeaderName>(),
    )?;
    m.add(
        "QiniuInvalidHeaderValue",
        py.get_type::<QiniuInvalidHeaderValue>(),
    )?;
    m.add(
        "QiniuEmptyChainCredentialsProvider",
        py.get_type::<QiniuEmptyChainCredentialsProvider>(),
    )?;
    m.add("QiniuJsonError", py.get_type::<QiniuJsonError>())?;
    m.add("QiniuTimeError", py.get_type::<QiniuTimeError>())?;
    m.add("QiniuBase64Error", py.get_type::<QiniuBase64Error>())?;
    m.add(
        "QiniuUploadTokenFormatError",
        py.get_type::<QiniuUploadTokenFormatError>(),
    )?;
    m.add(
        "QiniuUnsupportedTypeError",
        py.get_type::<QiniuUnsupportedTypeError>(),
    )?;
    m.add("QiniuIoError", py.get_type::<QiniuIoError>())?;
    Ok(())
}

create_exception!(qiniu_sdk_bindings, QiniuCallbackError, PyException);
create_exception!(qiniu_sdk_bindings, QiniuUnknownError, PyException);
create_exception!(qiniu_sdk_bindings, QiniuInvalidURLError, PyValueError);
create_exception!(qiniu_sdk_bindings, QiniuInvalidHeaderName, PyValueError);
create_exception!(qiniu_sdk_bindings, QiniuInvalidHeaderValue, PyValueError);
create_exception!(
    qiniu_sdk_bindings,
    QiniuEmptyChainCredentialsProvider,
    PyValueError
);
create_exception!(qiniu_sdk_bindings, QiniuJsonError, PyValueError);
create_exception!(qiniu_sdk_bindings, QiniuTimeError, PyValueError);
create_exception!(qiniu_sdk_bindings, QiniuBase64Error, PyValueError);
create_exception!(
    qiniu_sdk_bindings,
    QiniuUploadTokenFormatError,
    PyValueError
);
create_exception!(qiniu_sdk_bindings, QiniuUnsupportedTypeError, PyValueError);
create_exception!(qiniu_sdk_bindings, QiniuIoError, PyIOError);
