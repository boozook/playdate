use crate::ffi::PlaydateTilemap;


pub static TILEMAP: PlaydateTilemap = PlaydateTilemap { newTilemap: None,
                                                        freeTilemap: None,
                                                        setImageTable: None,
                                                        getImageTable: None,
                                                        setSize: None,
                                                        getSize: None,
                                                        getPixelSize: None,
                                                        setTiles: None,
                                                        setTileAtPosition: None,
                                                        getTileAtPosition: None,
                                                        drawAtPoint: None };
