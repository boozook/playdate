use crate::ffi::PlaydateVideoStream;


pub static VIDEOSTREAM: PlaydateVideoStream = PlaydateVideoStream { newPlayer: None,
                                                                    freePlayer: None,
                                                                    setBufferSize: None,
                                                                    setFile: None,
                                                                    setHTTPConnection: None,
                                                                    getFilePlayer: None,
                                                                    getVideoPlayer: None,
                                                                    update: None,
                                                                    getBufferedFrameCount: None,
                                                                    getBytesRead: None,
                                                                    setTCPConnection: None };
