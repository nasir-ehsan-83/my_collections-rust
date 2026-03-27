pub mod simple_sort;
pub mod comparison;

// Re-export for easier access
pub use comparison::insertion::insertion_sort;
pub use comparison::bubble::bubble_sort;
pub use simple_sort::selection_sort;