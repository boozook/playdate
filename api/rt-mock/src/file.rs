use crate::ffi::PlaydateFile;


pub static FILE: PlaydateFile = PlaydateFile { geterr: None,
                                               listfiles: None,
                                               stat: None,
                                               mkdir: None,
                                               unlink: None,
                                               rename: None,
                                               open: None,
                                               close: None,
                                               read: None,
                                               write: None,
                                               flush: None,
                                               tell: None,
                                               seek: None };
