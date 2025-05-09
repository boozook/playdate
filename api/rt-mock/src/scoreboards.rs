use crate::ffi::PlaydateScoreboards;


pub static SCOREBOARDS: PlaydateScoreboards = PlaydateScoreboards { addScore: None,
                                                                    getPersonalBest: None,
                                                                    freeScore: None,
                                                                    getScoreboards: None,
                                                                    freeBoardsList: None,
                                                                    getScores: None,
                                                                    freeScoresList: None };
