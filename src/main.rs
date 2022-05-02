//! # MDBook Preprocessor for Vlang
//! A preprocessor parsing MDBook compatible with Vlang
//! to enable preproccessing in Vlang
//!
//! The goal of this project is to allow preprocessing with Vlang
//! by formatting deserialized mdbook, and sending encoded object via rmb.
//! The processed book is then received via rmb and formatted back to 
//! MDBook conventions ready for compilation.

extern crate docopt;
extern crate mdbook;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate redis;

use docopt::Docopt;
use mdbook::{
    book::{Book, BookItem},
    errors::{Error, Result as MdResult},
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use redis::Commands;
static USAGE: &str = "
Usage:
    mdbook-vpreprocessor
    mdbook-vpreprocessor supports <supports>
";

static NAME: &str = "mdbook-vpreprocessor";

#[derive(Deserialize)]
struct Args {
    pub arg_supports: Option<String>,
}

#[derive(serde::Serialize)]
pub struct VBook {
    pub sections: Vec<VBookItem>,
    /* private fields */
}

#[derive(serde::Serialize)]
pub struct VBookItem {
    pub chapter: VChapter
}

#[derive(serde::Serialize)]
pub struct VChapter {
    pub name: String,
    pub content: String,
    pub sub_items: Vec<VBookItem>,
}

fn main() -> MdResult<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|a| a.deserialize())
        .unwrap_or_else(|e| e.exit());
    info!("Running presentation preprocessor");
    let pre = VPreProcessor;
    if let Some(ref arg) = args.arg_supports {
        debug!("just getting support info {:?}", arg);
        if pre.supports_renderer(arg) {
            ::std::process::exit(0);
        } else {
            ::std::process::exit(1);
        }
    }
    debug!("pre-processing");
    let (ctx, book) = CmdPreprocessor::parse_input(::std::io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(::std::io::stdout(), &processed_book)?;
    Ok(())
}

struct VPreProcessor;

impl Preprocessor for VPreProcessor {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        debug!("Running preprocessor");
        let res = send_book(&mut book);

        /*
        let vbook = format_book_to_vbook(book);
        const mb = new MessageBusClient(6379)

        const message = mb.prepare("mdbook.preprocess", 0)
        mb.send(message, vbook)
        mb.read(message, function (result) {
            console.log("result received")
            console.log(result)

            console.log("closing")
            process.exit(0)
        })
        */
        
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        match renderer {
            "html" => true,
            _ => false,
        }
    }
}

// gets a vector of bookitems 
// formats and returns a vection of vbookitems
fn sections_to_vsections<'a, I>(items: I) -> Vec<VBookItem>
where
    I: IntoIterator<Item = &'a mut BookItem> + 'a,
{
    //let mut sections: Vec<VBookItem>;
    let mut vbookitems = Vec::new();
    for item in items {
        if let BookItem::Chapter(ref mut ch) = *item {
            let vbookitem = VBookItem {
                chapter: VChapter {
                    name: ch.name.clone(),
                    content: ch.content.clone(),
                    sub_items: sections_to_vsections(&mut ch.sub_items)
                }
            }; 
            vbookitems.push(vbookitem);
        }
    }
    return vbookitems;
}

fn format_book_to_vbook(book: &mut Book) -> VBook{
    let vbook = VBook { sections: sections_to_vsections(&mut book.sections) };
    return vbook;
}

fn send_book(book: &mut Book) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let formatted = format_book_to_vbook(book);
    let mut con = client.get_connection()?;
    let encoded = serde_json::to_string(&formatted).unwrap();
    let _ : () = con.set("formatted_vbook", encoded)?;
    //return(con)
    Ok(())
}
