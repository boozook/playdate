use crate::ffi::PlaydateVideo;


pub static VIDEO: PlaydateVideo = PlaydateVideo { loadVideo: None,
                                                  freePlayer: None,
                                                  setContext: None,
                                                  useScreenContext: None,
                                                  renderFrame: None,
                                                  getError: None,
                                                  getInfo: None,
                                                  getContext: None };
