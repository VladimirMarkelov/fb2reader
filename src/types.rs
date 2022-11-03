pub struct AuthorInfo {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub nick_name: String,
}

pub struct BookInfo {
    pub title: String,
    pub genres: Vec<String>,
    pub seq_name: String,
    pub seq_num: String,
}

pub enum Element {
    Skip, // Unrecognized tag - caller can skip it
    Text(String),
    BeginStrong,
    EndStrong,
    BeginEm,
    EndEm,
    BeginSection,
    EndSection,
    BeginPara,
    EndPara,
    Author(AuthorInfo),
    Book(BookInfo),
    Image(String),
    NoteAnchor(String, String), // <a l:href="#n_2" type="note">[2]</a>
    EmptyLine,
    StartSub,
    EndSub,
    StartSup,
    EndSup,
    Title(String),
    SubTitle(String),
    StartNotes, // <body name="notes">
    EndNotes,
    StartNote(String), // <section id="n_1">
    EndNote,
    StartPoem,
    EndPoem,
    StartEpigraph,
    EndEpigraph,
    StartStanza,
    EndStanza,
    StartCite,
    EndCite,
    CiteAuthor(String), // cite or poem author
    StartAnnotation,
    EndAnnotation,
    // Binary, // Do not need it
    // Style, // Do not need it
    // All the rest tags are ignored
}
