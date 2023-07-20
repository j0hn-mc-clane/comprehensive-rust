// MORNING

fn multiply(x: i16, y:i16) -> i16 {
    x * y
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32;3]; 3] {
    let mut intermediary = [[0;3];3];
    let iter_matrix = matrix.into_iter();
    for (i, row) in iter_matrix.enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            intermediary[j][i] = col
        }
    }

    intermediary
}

fn pretty_print(matrix: &[[i32; 3];3]) {
    for row in matrix {
        println!("{row:?}");
    }
}


pub fn morning() {
    let x: i8 = 15;
    let y: i16 = 1_000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);

}


// AFTERNOON
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        todo!("Initialize and return a `Library` value")
    }

    fn len(self) -> usize {
        todo!("Return the length of `self.books`")
    }

    fn is_empty(self) -> bool {
        todo!("Return `true` if `self.books` is empty")
    }

    fn add_book(self, book: Book) {
        todo!("Add a new book to `self.books`")
    }

    fn print_books(self) {
        todo!("Iterate over `self.books` and each book's title and year")
    }

    fn oldest_book(self) -> Option<&Book> {
        todo!("Return a reference to the oldest book (if any)")
    }
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

pub fn afternoon() {
    let library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    
    
    library.print_books();
    
    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }
    
    println!("The library has {} books", library.len());
    library.print_books();
}