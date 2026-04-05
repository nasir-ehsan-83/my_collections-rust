pub mod comparison;

// Re-export for easier access
pub use comparison::insertion::insertion_sort;
pub use comparison::bubble::bubble_sort;
pub use comparison::selection::selection_sort;
pub use comparison::shell::shell_sort;
pub use comparison::gnome::gnome_sort;
pub use comparison::cocktail_shaker::cocktail_shaker_sort;
pub use comparison::odd_even::odd_even_sort;
pub use comparison::comb::comb_sort;
pub use comparison::tree::tree_sort;
pub use comparison::heap::heap_sort;