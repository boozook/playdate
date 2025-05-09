use crate::ffi::PlaydateHttp;
use crate::ffi::PlaydateNetwork;
use crate::ffi::PlaydateTcp;


pub static NETWORK: PlaydateNetwork = PlaydateNetwork { http: HTTP,
                                                        tcp: TCP,
                                                        getStatus: None,
                                                        setEnabled: None,
                                                        reserved: Default::default() };

static TCP: PlaydateTcp = PlaydateTcp {};
static HTTP: PlaydateHttp = PlaydateHttp {};
