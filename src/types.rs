pub struct AuthorInfo {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub nick_name: String,
}

impl Default for AuthorInfo {
    fn default() -> AuthorInfo {
        AuthorInfo {
            first_name: String::new(),
            last_name: String::new(),
            middle_name: String::new(),
            nick_name: String::new(),
        }
    }
}

pub struct BookInfo {
    pub title: String,
    pub genres: Vec<String>,
    pub seq_name: String,
    pub seq_num: String,
}

impl Default for BookInfo {
    fn default() -> BookInfo {
        BookInfo {
            title: String::new(),
            genres: Vec::new(),
            seq_name: String::new(),
            seq_num: String::new(),
        }
    }
}

pub struct Book {
    pub author: AuthorInfo,
    pub info: BookInfo,
    pub text: Vec<Element>,
}

impl Default for Book {
    fn default() -> Book {
        Book {
            author: Default::default(),
            info: Default::default(),
            text: Vec::new(),
        }
    }
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
                                // <a xlink:href="#N2" type="note">2</a></p>
    EmptyLine,
    StartSub,
    EndSub,
    StartSup,
    EndSup,
    Title(String),
    SubTitle(String),
    StartNotes, // <body name="notes"> or // <body name="footnotes">
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
