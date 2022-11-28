use crate::aliases::PageBuffer;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub(crate) struct PageNotFound;

pub(crate) type ReadPageResult = Result<PageBuffer, PageNotFound>;