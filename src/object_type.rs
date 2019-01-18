/// The type of object in the graph this refers to.
#[derive(Debug, Clone)]
pub enum ObjectType {
  /// A song.
  MusicSong,
  /// A music album.
  MusicAlbum,
  /// A music playlist.
  MusicPlaylist,
  /// A radio station
  MusicRadioStation,
  /// A movie
  VideoMovie,
  /// An episode of a show.
  VideoEpisode,
  /// A TV show.
  VideoTvShow,
  /// Miscellaneous video.
  VideoOther,
  /// An article.
  Article,
  /// A book.
  Book,
  /// A profile.
  Profile,
  /// A website.
  Website,
}
