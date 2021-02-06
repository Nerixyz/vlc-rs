// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.


/// Safety: Expect both enums to have the same size (repr(i32))
macro_rules! libvlc_enum {
    ($libvlc_enum:ident,$enum_name:ident) => {
        impl From<libvlc_sys::$libvlc_enum> for $enum_name {
            fn from(other: libvlc_sys::$libvlc_enum) -> Self {
                unsafe {
                    std::mem::transmute(other)
                }
            }
        }

        impl Into<libvlc_sys::$libvlc_enum> for $enum_name {
            fn into(self) -> libvlc_sys::$libvlc_enum {
                unsafe {
                    std::mem::transmute(self)
                }
            }
        }
    };
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogLevel {
    Debug = 0,
    Notice = 2,
    Warning = 3,
    Error = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Meta {
    Title = 0,
    Artist = 1,
    Genre = 2,
    Copyright = 3,
    Album = 4,
    TrackNumber = 5,
    Description = 6,
    Rating = 7,
    Date = 8,
    Setting = 9,
    URL = 10,
    Language = 11,
    NowPlaying = 12,
    Publisher = 13,
    EncodedBy = 14,
    ArtworkURL = 15,
    TrackID = 16,
    TrackTotal = 17,
    Director = 18,
    Season = 19,
    Episode = 20,
    ShowName = 21,
    Actors = 22,
}

libvlc_enum!(libvlc_meta_t, Meta);

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum State {
    NothingSpecial = 0,
    Opening = 1,
    Buffering = 2,
    Playing = 3,
    Paused = 4,
    Stopped = 5,
    Ended = 6,
    Error = 7,
}

libvlc_enum!(libvlc_state_t, State);

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TrackType {
    Unknown = -1,
    Audio   = 0,
    Video   = 1,
    Text    = 2
}

libvlc_enum!(libvlc_track_type_t, TrackType);

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Position {
    Disable = -1,
    Center,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
}

libvlc_enum!(libvlc_position_t, Position);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VideoAdjustOption {
    Enable = 0,
    Contrast,
    Brightness,
    Hue,
    Saturation,
    Gamma
}

// #[repr(C)]
// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub enum ParseFlag {
//     ParseLocal,
//     ParseNetwork,
//     FetchLocal,
//     FetchNetwork,
// }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EventType {
    MediaMetaChanged = 0,
    MediaSubItemAdded,
    MediaDurationChanged,
    MediaParsedChanged,
    MediaFreed,
    MediaStateChanged,
    MediaSubItemTreeAdded,

    MediaPlayerMediaChanged = 0x100,
    MediaPlayerNothingSpecial,
    MediaPlayerOpening,
    MediaPlayerBuffering,
    MediaPlayerPlaying,
    MediaPlayerPaused,
    MediaPlayerStopped,
    MediaPlayerForward,
    MediaPlayerBackward,
    MediaPlayerEndReached,
    MediaPlayerEncounteredError,
    MediaPlayerTimeChanged,
    MediaPlayerPositionChanged,
    MediaPlayerSeekableChanged,
    MediaPlayerPausableChanged,
    MediaPlayerTitleChanged,
    MediaPlayerSnapshotTaken,
    MediaPlayerLengthChanged,
    MediaPlayerVout,
    MediaPlayerScrambledChanged,

    MediaListItemAdded = 0x200,
    MediaListWillAddItem,
    MediaListItemDeleted,
    MediaListWillDeleteItem,

    MediaListViewItemAdded = 0x300,
    MediaListViewWillAddItem,
    MediaListViewItemDeleted,
    MediaListViewWillDeleteItem,

    MediaListPlayerPlayed = 0x400,
    MediaListPlayerNextItemSet,
    MediaListPlayerStopped,

    MediaDiscovererStarted = 0x500,
    MediaDiscovererEnded,

    VlmMediaAdded = 0x600,
    VlmMediaRemoved,
    VlmMediaChanged,
    VlmMediaInstanceStarted,
    VlmMediaInstanceStopped,
    VlmMediaInstanceStatusInit,
    VlmMediaInstanceStatusOpening,
    VlmMediaInstanceStatusPlaying,
    VlmMediaInstanceStatusPause,
    VlmMediaInstanceStatusEnd,
    VlmMediaInstanceStatusError
}
