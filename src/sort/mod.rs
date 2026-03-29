pub mod comparison;

// Re-export for easier access
pub use comparison::insertion::insertion_sort;
pub use comparison::bubble::bubble_sort;
pub use comparison::selection::selection_sort;
pub use comparison::shell::shell_sort;