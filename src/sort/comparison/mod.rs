mod divide_conquer;
mod exchange;
mod insertion;
mod selection;
mod tree;

pub use tree::tree_sort;


pub use divide_conquer::{
    merge::{
        merge_sort,
         merge
    },
    quick::quick_sort
};
pub use exchange::{
    bubble::bubble_sort,
    cocktail_shaker::cocktail_shaker_sort,
    comb::comb_sort,
    gnome::gnome_sort,
    odd_even::odd_even_sort
};
pub use insertion::{
    insertion::insertion_sort,
    shell::shell_sort
};
pub use selection::{
    pancake::pancake_sort,
    selection::selection_sort,
    heap::heap_sort
};