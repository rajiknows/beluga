//=============================== MACROS ONLY SSG ===============================================

// so the idea is that we will have a index.html where we will put some html but
//
// along with that we will add our own syntax inside which we will write rust code that will fetch
// the content and put it there
//
//
// all of this needs to be done at comptime.... lessgo

use beluga_macros::render;

fn main() {
    let html = render!();
    println!("{}", html);
}
